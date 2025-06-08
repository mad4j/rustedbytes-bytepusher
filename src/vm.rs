use minifb::Window;
use rodio::Sink;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::audio::AudioHandler;
use crate::cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keyboard::KeyboardHandler;
use crate::screen::ScreenHandler;

pub struct VirtualMachine {
    pub window: Rc<RefCell<Window>>,
    pub cpu: Cpu,
    pub audio_handler: AudioHandler,
    pub sink: Rc<RefCell<Sink>>,
    pub keyboard_handler: KeyboardHandler,
    pub screen_handler: ScreenHandler,
    pub frame_duration: Duration,
}

impl VirtualMachine {
    pub fn tick_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();

        self.keyboard_handler.handle_events();

        self.cpu.frame_tick();

        self.audio_handler.append_samples();

        let new_frame = self.cpu.get_screen_buffer();
        self.screen_handler.render(&new_frame);

        self.window.borrow_mut().update_with_buffer(
            self.screen_handler.get_screen(),
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )?;

        let elapsed = start.elapsed();
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        } else {
            eprintln!("Frame took too long: {:?}", elapsed);
        }
        Ok(())
    }
}
