use chrono::Utc;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::{env, fs, io::Write, path::Path};

mod cpu;
mod audio;
use crate::{
    cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH},
    audio::SampleBufferSource,
};

fn key_to_hex(key: Key) -> Option<u8> {
    match key {
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xC),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = Cpu::default();

    let filename = env::args().nth(1).ok_or("usage: kpsh FILE_PATH")?;
    let rom_as_vec = fs::read(&filename)?;

    let mut window = Window::new(
        &format!("kpsh - {}", filename),
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    cpu.load_rom(&rom_as_vec);

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let frame_duration = std::time::Duration::from_millis(16);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start = std::time::Instant::now();

        for key in window.get_keys_pressed(KeyRepeat::No) {
            if let Some(hex) = key_to_hex(key) {
                cpu.keys[hex as usize] = true;
            }
        }

        for key in window.get_keys_released() {
            if let Some(hex) = key_to_hex(key) {
                cpu.keys[hex as usize] = false;
            }
        }

        if window.is_key_pressed(Key::M, KeyRepeat::No) {
            let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            let rom_filename = Path::new(&filename).file_stem().unwrap();

            let mut file = fs::File::create(format!(
                "kpsh_{}_{}.BytePusher",
                rom_filename.to_string_lossy(),
                timestamp
            ))?;
            file.write_all(cpu.memory.as_slice())?;
        }

        cpu.tick();
        if cpu.sample_buffer.iter().any(|&sample| sample != 0) {
            sink.append(SampleBufferSource::from(cpu.sample_buffer));
        }

        window.update_with_buffer(&cpu.screen, SCREEN_WIDTH, SCREEN_HEIGHT)?;

        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
    Ok(())
}
