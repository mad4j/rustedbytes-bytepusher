use minifb::Window;
use rodio::Sink;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::audio::AudioHandler;
use crate::cpu::{Cpu, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keyboard::KeyboardHandler;
use crate::memory::Memory;
use crate::screen::ScreenHandler;

pub struct VirtualMachine {
    pub window: Rc<RefCell<Window>>,
    pub sink: Rc<RefCell<Sink>>,
    pub cpu: Cpu,
    pub memory: Rc<RefCell<Memory>>,
    pub audio_handler: AudioHandler,
    pub keyboard_handler: KeyboardHandler,
    pub screen_handler: ScreenHandler,
    pub frame_duration: Duration,
}

impl VirtualMachine {
    pub fn new(
        window: Rc<RefCell<Window>>,
        sink: Rc<RefCell<Sink>>,
        frame_duration: Duration,
    ) -> Self {
        let memory = Rc::new(RefCell::new(Memory::new(crate::cpu::MEMORY_SIZE)));
        let cpu = Cpu::new(Rc::clone(&memory));
        let audio_handler =
            AudioHandler::new(crate::cpu::AUDIO_REGISTER_ADDR, Rc::clone(&memory), Rc::clone(&sink));
        let keyboard_handler = KeyboardHandler::new(
            crate::cpu::KEYBOARD_REGISTER_ADDR,
            Rc::clone(&window),
            Rc::clone(&memory),
        );
        let screen_handler = ScreenHandler::new();
        Self {
            window,
            sink,
            cpu,
            memory,
            audio_handler,
            keyboard_handler,
            screen_handler,
            frame_duration,
        }
    }


    pub fn load_rom(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rom_data = std::fs::read(file_path)?;
        self.memory.borrow_mut().copy_from(0, &rom_data);
        Ok(())
    }

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
