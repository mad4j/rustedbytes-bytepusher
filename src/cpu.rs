use crate::memory::Memory;

const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16MiB

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
    pub memory: Memory,
}

impl Default for Cpu {
    fn default() -> Self {
        // Initialize the CPU with a program counter and memory
        Self {
            program_counter: 0x200,
            memory: Memory::new(MEMORY_SIZE),
        }
    }
}

impl Cpu {

    fn execute_instruction(&mut self) {

        let pc = self.program_counter;

        let (addr_a, addr_b, addr_jump) = (
            self.memory.read_24_bits(pc),
            self.memory.read_24_bits(pc + 3),
            self.memory.read_24_bits(pc + 6),
        );

        self.memory[addr_b] = self.memory[addr_a];
        self.program_counter = addr_jump;
    }


    #[inline(always)]
    pub fn frame_tick(&mut self) {
        // program counter update from memory at the beginnging of each frame
        self.program_counter = self.memory.read_24_bits(PROGRAM_COUNTER_ADDR);

        for _ in 0..65536 {
            self.execute_instruction();
        }
    }


    pub fn update_keyboard_state(&mut self, key_values: u16) {
        self.memory.write_16_bits(KEYBOARD_REGISTER_ADDR, key_values);
    }


    pub fn get_screen_buffer(&self) -> &[u8; SCREEN_BUFFER_SIZE] {
        let graphics_addr = (self.memory[SCREEN_REGISTER_ADDR] as usize) << 16;
        let new_frame =  &self.memory[graphics_addr..graphics_addr + SCREEN_BUFFER_SIZE];

        new_frame.try_into().unwrap()
    }

    pub fn get_sample_buffer(&mut self) -> &[u8; AUDIO_BUFFER_SIZE] {
        let audio_addr = (self.memory.read_16_bits(AUDIO_REGISTER_ADDR) as usize) << 8;
        let sample_buffer = &self.memory[audio_addr..audio_addr + AUDIO_BUFFER_SIZE];

        sample_buffer.try_into().unwrap()
    }
}