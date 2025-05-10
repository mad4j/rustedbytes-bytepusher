use chrono::{DateTime, Utc};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::{env, fs, io::Write, path::Path};

mod emu;
mod sound;
use crate::{
    emu::{Emulator, SCREEN_HEIGHT, SCREEN_WIDTH},
    sound::SampleBufferSource,
};

const BLANK_BUFFER: [u8; 256] = [0; 256];

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

fn main() -> Result<(), minifb::Error> {
    let mut emu = Emulator::default();

    let filename = env::args().nth(1).expect("usage: kpsh FILE_PATH");

    let rom_as_vec: Vec<u8> = fs::read(&filename).expect("unable to open rom file");

    let Ok(mut window) = Window::new(
        format!("kpsh - {}", filename).as_str(),
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) else {
        println!("creation of window failed.");
        return Ok(());
    };
    window.set_target_fps(60);

    emu.load_rom(rom_as_vec);

    let (_stream, stream_handle) =
        OutputStream::try_default().expect("unable to create audio output stream");
    let sink = Sink::try_new(&stream_handle).expect("unable to create audio output sink");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for key in window.get_keys_pressed(KeyRepeat::No) {
            if let Some(hex) = key_to_hex(key) {
                emu.keys[hex as usize] = true;
            }
        }

        for key in window.get_keys_released() {
            if let Some(hex) = key_to_hex(key) {
                emu.keys[hex as usize] = false;
            }
        }

        if window.is_key_pressed(Key::M, KeyRepeat::No) {
            let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            let rom_filename = Path::new(&filename).file_stem().unwrap();

            if let Ok(mut file) = fs::File::create(format!(
                "kpsh_{}_{}.BytePusher",
                rom_filename.to_string_lossy(),
                timestamp
            )) {
                file.write(emu.memory.as_slice()).unwrap();
            }
        }

        emu.tick();
        if emu.sample_buffer != BLANK_BUFFER {
            sink.append(SampleBufferSource::from(emu.sample_buffer));
        }

        window.update_with_buffer(&emu.screen, SCREEN_WIDTH, SCREEN_HEIGHT)?;
    }
    Ok(())
}
