use clap::Parser;
use minifb::{Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};

use std::cell::RefCell;
use std::rc::Rc;

mod audio;
mod cpu;
mod keyboard;
mod memory;
mod screen;
mod vm;

use crate::vm::SCREEN_HEIGHT;
use crate::vm::SCREEN_WIDTH;

/// BytePusher VM
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Percorso del file ROM BytePusher
    rom: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let window = Rc::new(RefCell::new(Window::new(
        &format!("RustedBytes - BytePusherVM"),
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )?));

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Rc::new(RefCell::new(Sink::try_new(&stream_handle)?));

    let mut vm = vm::VirtualMachine::new(Rc::clone(&window), Rc::clone(&sink));

    vm.load_rom(&args.rom)?;
    vm.run()?;

    Ok(())
}
