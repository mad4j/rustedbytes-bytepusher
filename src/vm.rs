use minifb::Window;
use rodio::Sink;
use std::time::{Duration, Instant};

use crate::audio::AudioHandler;
use crate::cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keyboard::KeyboardHandler;
use crate::screen::ScreenHandler;

pub struct VirtualMachine {
    pub window: Window,
    pub cpu: Cpu,
    pub audio_handler: AudioHandler,
    pub sink: Sink,
    pub keyboard_handler: KeyboardHandler,
    pub screen_handler: ScreenHandler,
    pub frame_duration: Duration,
}

impl VirtualMachine {
    pub fn tick_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();

        self.keyboard_handler.handle_events(&self.window);
        self.cpu.update_keyboard_state(self.keyboard_handler.get_keyboard_state());

        self.cpu.tick();

        let new_sample_buffer = self.cpu.get_sample_buffer();
        self.audio_handler.append_buffer_to_sink(&self.sink, new_sample_buffer);

        let new_frame = self.cpu.get_screen_buffer();
        self.screen_handler.render(new_frame);

        self.window.update_with_buffer(self.screen_handler.get_screen(), SCREEN_WIDTH, SCREEN_HEIGHT)?;

        let elapsed = start.elapsed();
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        }
        Ok(())
    }
}
