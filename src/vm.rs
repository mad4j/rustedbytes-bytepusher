use minifb::Window;
use rodio::Sink;
use std::time::{Duration, Instant};

use crate::audio::AudioHandler;
use crate::cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keyboard::KeyboardHandler;
use crate::screen::ScreenHandler;

pub fn run_vm_frame(
    window: &mut Window,
    cpu: &mut Cpu,
    audio_handler: &AudioHandler,
    sink: &Sink,
    keyboard_handler: &mut KeyboardHandler,
    screen_handler: &mut ScreenHandler,
    frame_duration: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    keyboard_handler.handle_events(window);
    cpu.update_keyboard_state(keyboard_handler.get_keyboard_state());

    cpu.tick();

    let new_sample_buffer = cpu.get_sample_buffer();
    audio_handler.append_buffer_to_sink(sink, new_sample_buffer);

    let new_frame = cpu.get_screen_buffer();
    screen_handler.render(new_frame);

    window.update_with_buffer(screen_handler.get_screen(), SCREEN_WIDTH, SCREEN_HEIGHT)?;

    let elapsed = start.elapsed();
    if elapsed < frame_duration {
        std::thread::sleep(frame_duration - elapsed);
    }
    Ok(())
}
