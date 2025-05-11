use chrono::Utc;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::{env, fs, io::Write, path::Path};

mod cpu;
mod audio;
mod keyboard;

use crate::{
    cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH},
    audio::AudioHandler,
    keyboard::KeyboardHandler,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = Cpu::default();
    let mut audio_handler = AudioHandler::new();
    let mut keyboard_handler = KeyboardHandler::new();

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

        let key_values = keyboard_handler.get_key_values(&window);

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

        cpu.tick(key_values);
        audio_handler.update_sample_buffer(&cpu.memory);
        audio_handler.append_to_sink(&sink);

        window.update_with_buffer(&cpu.screen, SCREEN_WIDTH, SCREEN_HEIGHT)?;

        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
    Ok(())
}
