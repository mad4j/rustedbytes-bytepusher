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
    pub frame_duration: Duration,
}


/// Implementation of the VirtualMachine
/// This struct encapsulates the VM's components and provides methods to load ROMs and process frames.
/// It includes the CPU, memory, audio handler, keyboard handler, and screen handler.
impl VirtualMachine {
    pub fn new(
        window: Rc<RefCell<Window>>,
        sink: Rc<RefCell<Sink>>,
        frame_duration: Duration,
    ) -> Self {

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
            frame_duration,
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

        // Measure the time taken for the frame processing
        let start = Instant::now();

        // Handle keyboard events
        self.keyboard_handler.handle_events();

        // process the CPU instructions
        self.cpu.tick();

        // Append audio samples from the memory to the audio sink
        self.audio_handler.append_samples();

        // Render the screen frame
        self.screen_handler.render_frame()?;

        // wait for the next frame duration
        // This is to ensure that each frame takes approximately the same amount of time
        let elapsed = start.elapsed();
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        } else {
            eprintln!("Frame took too long: {:?}", elapsed);
        }

        Ok(())
    }
}
