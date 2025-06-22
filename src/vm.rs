use minifb::Window;
use rodio::Sink;
use spin_sleep::SpinSleeper;
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

pub const FRAME_RATE: u32 = 60; // 60 frames per second

/// VirtualMachine struct encapsulates the components of the BytePusher VM.
/// It includes the CPU, memory, audio handler, keyboard handler, and screen handler.
pub struct VirtualMachine {
    pub _window: Rc<RefCell<Window>>,
    pub _sink: Rc<RefCell<Sink>>,
    pub cpu: Cpu,
    pub memory: Rc<RefCell<Memory>>,
    pub audio_handler: AudioHandler,
    pub keyboard_handler: KeyboardHandler,
    pub screen_handler: ScreenHandler,
}

/// Implementation of the VirtualMachine
/// This struct encapsulates the VM's components and provides methods to load ROMs and process frames.
/// It includes the CPU, memory, audio handler, keyboard handler, and screen handler.
impl VirtualMachine {
    pub fn new(window: Rc<RefCell<Window>>, sink: Rc<RefCell<Sink>>) -> Self {
        // Initialize the memory with a size of MEMORY_SIZE
        let memory = Rc::new(RefCell::new(Memory::new(MEMORY_SIZE)));

        // Initialize the CPU with the memory
        let cpu = Cpu::new(Rc::clone(&memory));

        // Initialize the audio handler with the memory and sink
        let audio_handler =
            AudioHandler::new(AUDIO_REGISTER_ADDR, Rc::clone(&memory), Rc::clone(&sink));

        // Initialize the keyboard handler with the memory and window
        let keyboard_handler = KeyboardHandler::new(
            KEYBOARD_REGISTER_ADDR,
            Rc::clone(&window),
            Rc::clone(&memory),
        );

        // Initialize the screen handler with the memory and window
        let screen_handler =
            ScreenHandler::new(SCREEN_REGISTER_ADDR, Rc::clone(&memory), Rc::clone(&window));

        // Return the new VirtualMachine instance
        Self {
            _window: window,
            _sink: sink,
            cpu,
            memory,
            audio_handler,
            keyboard_handler,
            screen_handler,
        }
    }

    /// Load a ROM file into the VM's memory.
    pub fn load_rom(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rom_data = std::fs::read(file_path)?;
        self.memory.borrow_mut().copy_from(0, &rom_data);
        Ok(())
    }

    /// Process a single frame of the VM, handling input, CPU ticks, audio, and rendering.
    pub fn process_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Handle keyboard events
        self.keyboard_handler.handle_events();

        // process the CPU instructions
        self.cpu.tick();

        // Append audio samples from the memory to the audio sink
        self.audio_handler.append_samples();

        // Render the screen frame
        self.screen_handler.render_frame()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let frame_duration = Duration::from_secs_f64(1.0 / FRAME_RATE as f64);
        let sleeper = SpinSleeper::default();
        let mut next_frame = Instant::now() + frame_duration;
        let mut screenshot_index = 1;
        // Main loop for the VM
        while self._window.borrow().is_open()
            && !self._window.borrow().is_key_down(minifb::Key::Escape)
        {
            self.process_frame()?;
            // Salva screenshot se il tasto S è premuto
            if self._window.borrow().is_key_pressed(minifb::Key::S, minifb::KeyRepeat::No) {
                self.screen_handler.save_screen_png(screenshot_index)?;
                screenshot_index += 1;
            }
            let now = Instant::now();
            if now < next_frame {
                sleeper.sleep(next_frame - now);
            } else {
                eprintln!(
                    "Frame took too long: {:.02?}",
                    frame_duration + (now - next_frame)
                );
            }
            next_frame += frame_duration;
        }
        Ok(())
    }
}
