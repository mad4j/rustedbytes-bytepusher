const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16MiB

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;

pub struct Cpu {
    pub memory: Vec<u8>,
}

impl Default for Cpu {
    fn default() -> Self {
        // Initialize the CPU with memory only
        Self {
            memory: vec![0; MEMORY_SIZE],
        }
    }
}

impl Cpu {
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[..rom.len()].copy_from_slice(rom);
    }

    #[inline(always)]
    fn read_24_bits(&self, slice: &[u8]) -> usize {
        (u32::from_be_bytes([slice[0], slice[1], slice[2], 0]) >> 8) as usize
    }

    fn execute_instruction(&mut self) {
        // Usa il program counter dalla memoria (indirizzi 2,3,4)
        let pc = self.read_24_bits(&self.memory[2..5]);
        let (addr_a, addr_b, addr_jump) = (
            self.read_24_bits(&self.memory[pc..pc + 3]),
            self.read_24_bits(&self.memory[pc + 3..pc + 6]),
            self.read_24_bits(&self.memory[pc + 6..pc + 9]),
        );

        self.memory[addr_b] = self.memory[addr_a];
        // Aggiorna il program counter in memoria
        let jump_bytes = (addr_jump as u32).to_be_bytes();
        self.memory[2] = jump_bytes[1];
        self.memory[3] = jump_bytes[2];
        self.memory[4] = jump_bytes[3];
    }

    pub fn update_keyboard_state(&mut self, key_values: u16) {
        self.memory[0..2].copy_from_slice(&key_values.to_be_bytes());
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        // Nessun program_counter nel campo struct, già letto da memoria
        for _ in 0..65536 {
            self.execute_instruction();
        }
    }

    pub fn get_screen_buffer(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        let graphics_addr = (self.memory[5] as usize) << 16;
        let new_frame = {
            let frame_slice = &self.memory[graphics_addr..graphics_addr + 65536];
            frame_slice.try_into().unwrap()
        };

        new_frame
    }

    pub fn get_sample_buffer(&mut self) -> &[u8; 256] {
        let audio_addr = self.memory[6] as usize * 65536 + self.memory[7] as usize * 256;
        let sample_buffer = &self.memory[audio_addr..audio_addr + 256];

        sample_buffer.try_into().unwrap()
    }
}
