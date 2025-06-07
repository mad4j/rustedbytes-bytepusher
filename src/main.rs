use clap::Parser;
use minifb::{Key, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::fs;

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

/// BytePusher VM
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Percorso del file ROM BytePusher
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let window = Window::new(
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
    let keyboard_handler = KeyboardHandler::new();
    let screen_handler = screen::ScreenHandler::new();

    let rom_as_vec = fs::read(&args.rom)?;
    cpu.load_rom(&rom_as_vec);

    let frame_duration = std::time::Duration::from_millis(16);

    let mut vm = vm::VirtualMachine {
        window,
        cpu,
        audio_handler,
        sink,
        keyboard_handler,
        screen_handler,
        frame_duration,
    };

    while vm.window.is_open() && !vm.window.is_key_down(Key::Escape) {
        vm.tick_frame()?;
    }
    Ok(())
}
