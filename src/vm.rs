use minifb::Window;
use rodio::Sink;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::audio::AudioHandler;
use crate::cpu::Cpu;
use crate::keyboard::KeyboardHandler;
use crate::memory::Memory;
use crate::screen::ScreenHandler;

pub const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16 MiB of memory

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;
pub const SCREEN_BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const AUDIO_BUFFER_SIZE: usize = 256;
pub const AUDIO_SAMPLES_PER_SECOND: u32 = 15360; // 256 * 60

pub const KEYBOARD_REGISTER_ADDR: usize = 0x000000;
pub const PROGRAM_COUNTER_ADDR: usize = 0x000002;
pub const SCREEN_REGISTER_ADDR: usize = 0x000005;
pub const AUDIO_REGISTER_ADDR: usize = 0x000006;

pub struct VirtualMachine {
    pub _window: Rc<RefCell<Window>>,
    pub _sink: Rc<RefCell<Sink>>,
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
        let memory = Rc::new(RefCell::new(Memory::new(MEMORY_SIZE)));
        let cpu = Cpu::new(Rc::clone(&memory));
        let audio_handler =
            AudioHandler::new(AUDIO_REGISTER_ADDR, Rc::clone(&memory), Rc::clone(&sink));
        let keyboard_handler = KeyboardHandler::new(
            KEYBOARD_REGISTER_ADDR,
            Rc::clone(&window),
            Rc::clone(&memory),
        );
        let screen_handler =
            ScreenHandler::new(SCREEN_REGISTER_ADDR, Rc::clone(&memory), Rc::clone(&window));
        Self {
            _window: window,
            _sink: sink,
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

    pub fn process_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();

        self.keyboard_handler.handle_events();

        self.cpu.tick();

        self.audio_handler.append_samples();

        self.screen_handler.render_frame()?;

        let elapsed = start.elapsed();

        //TODO: usare invece il limitatore di minifb
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        } else {
            eprintln!("Frame took too long: {:?}", elapsed);
        }
        Ok(())
    }
}
