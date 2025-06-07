use crate::memory::Memory;

const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16MiB

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;

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

    pub fn update_keyboard_state(&mut self, key_values: u16) {
        self.memory.write_16_bits(0, key_values);
    }

    #[inline(always)]
    pub fn frame_tick(&mut self) {
        // program counter update from memory at the beginnging of the frame
        self.program_counter = self.memory.read_24_bits(2);

        for _ in 0..65536 {
            self.execute_instruction();
        }
    }

    pub fn get_screen_buffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        let graphics_addr = (self.memory[5] as usize) << 16;
        let new_frame = {
            let frame_slice = &self.memory.data[graphics_addr..graphics_addr + 65536];
            frame_slice.try_into().unwrap()
        };

        new_frame
    }

    pub fn get_sample_buffer(&mut self) -> &[u8; 256] {
        let audio_addr = self.memory[6] as usize * 65536 + self.memory[7] as usize * 256;
        let sample_buffer = &self.memory.data[audio_addr..audio_addr + 256];

        sample_buffer.try_into().unwrap()
    }
}