use minifb::{Key, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::{env, fs};

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
        &format!("RustedBytes - BytePusher - {}", filename),
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
    
        keyboard_handler.handle_events(&window);
        cpu.update_keyboard_state(keyboard_handler.get_keyboard_state());

        cpu.tick();

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
