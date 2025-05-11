use minifb::{Key, Scale, Window, WindowOptions};
use rodio::{OutputStream, Sink};
use std::{env, fs};

mod cpu;
mod audio;
mod keyboard;
mod screen;

use crate::{
    cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH},
    audio::AudioHandler,
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
        let start = std::time::Instant::now();
    
        keyboard_handler.handle_events(&window);
        cpu.update_keyboard_state(keyboard_handler.get_keyboard_state());

        cpu.tick();

        let new_sample_buffer = cpu.get_sample_buffer();
        audio_handler.append_to_sink(&sink, new_sample_buffer);

        let new_frame = cpu.get_screen_buffer();
        screen_handler.render(new_frame);

        window
            .update_with_buffer(screen_handler.get_screen(), SCREEN_WIDTH, SCREEN_HEIGHT)?;

        let elapsed = start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
    Ok(())
}
