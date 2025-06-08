use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

pub const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16 MiB of memory

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;

pub const AUDIO_BUFFER_SIZE: usize = 256;
pub const SCREEN_BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const KEYBOARD_REGISTER_ADDR: usize = 0x000000;
pub const PROGRAM_COUNTER_ADDR: usize = 0x000002;
pub const SCREEN_REGISTER_ADDR: usize = 0x000005;
pub const AUDIO_REGISTER_ADDR: usize = 0x000006;

pub struct Cpu {
    program_counter: usize,
    pub memory: Rc<RefCell<Memory>>,
}

impl Default for Cpu {
    fn default() -> Self {
        // Initialize the CPU with a program counter and memory
        Self {
            program_counter: 0x200,
            memory: Rc::new(RefCell::new(Memory::new(MEMORY_SIZE))),
        }
    }
}

impl Cpu {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Self {
        Self {
            program_counter: 0x200,
            memory,
        }
    }

    #[inline(always)]
    fn execute_instruction(&mut self) {
        let pc = self.program_counter;
        let memory = &mut *self.memory.borrow_mut();
        let (addr_a, addr_b, addr_jump) = (
            memory.read_24_bits(pc),
            memory.read_24_bits(pc + 3),
            memory.read_24_bits(pc + 6),
        );
        memory[addr_b] = memory[addr_a];
        self.program_counter = addr_jump;
    }

    #[inline(always)]
    pub fn frame_tick(&mut self) {
        // needed to reduce borrow checker issues
        let pc = {
            let memory = &*self.memory.borrow();
            memory.read_24_bits(PROGRAM_COUNTER_ADDR)
        };

        self.program_counter = pc;
        for _ in 0..65536 {
            self.execute_instruction();
        }
    }

    pub fn get_screen_buffer(&self) -> [u8; SCREEN_BUFFER_SIZE] {
        let mem = self.memory.borrow();
        let graphics_addr = (mem[SCREEN_REGISTER_ADDR] as usize) << 16;
        let new_frame = &mem[graphics_addr..graphics_addr + SCREEN_BUFFER_SIZE];
        let mut arr = [0u8; SCREEN_BUFFER_SIZE];
        arr.copy_from_slice(new_frame);
        arr
    }

    pub fn get_sample_buffer(&self) -> [u8; AUDIO_BUFFER_SIZE] {
        let mem = self.memory.borrow();
        let audio_addr = (mem.read_16_bits(AUDIO_REGISTER_ADDR) as usize) << 8;
        let sample_buffer = &mem[audio_addr..audio_addr + AUDIO_BUFFER_SIZE];
        let mut arr = [0u8; AUDIO_BUFFER_SIZE];
        arr.copy_from_slice(sample_buffer);
        arr
    }
}
