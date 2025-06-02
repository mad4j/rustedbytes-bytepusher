use minifb::{Key, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::{env, fs};

mod audio;
mod cpu;
mod keyboard;
mod screen;
mod vm;

use crate::{
    audio::AudioHandler,
    cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::KeyboardHandler,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(
        &format!("RustedBytes - BytePusher "),
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?;

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let mut cpu = Cpu::default();
    let audio_handler = AudioHandler::new();
    let mut keyboard_handler = KeyboardHandler::new();
    let mut screen_handler = screen::ScreenHandler::new();

    let filename = env::args().nth(1).ok_or("usage: kpsh FILE_PATH")?;
    let rom_as_vec = fs::read(&filename)?;

    cpu.load_rom(&rom_as_vec);

    let frame_duration = std::time::Duration::from_millis(16);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        vm::run_vm_frame(
            &mut window,
            &mut cpu,
            &audio_handler,
            &sink,
            &mut keyboard_handler,
            &mut screen_handler,
            frame_duration,
        )?;
    }
    Ok(())
}
