const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16MiB

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;

pub struct Cpu {
    program_counter: usize,
    pub memory: Vec<u8>,
}

impl Default for Cpu {
    fn default() -> Self {
        // Generate palette attribute beforehand (so we don't have to parse it every time)
        let mut palette: [u32; 256] = [0; 256];
        for (idx, val) in palette.iter_mut().enumerate() {
            if idx >= 216 {
                break;
            }
            *val = ((idx as u32 / 36 * 0x33) << 16)
                | ((idx as u32 / 6 % 6 * 0x33) << 8)
                | (idx as u32 % 6 * 0x33);
        }

        Self {
            program_counter: 0x200,
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
        let pc = self.program_counter;
        let (addr_a, addr_b, addr_jump) = (
            self.read_24_bits(&self.memory[pc..pc + 3]),
            self.read_24_bits(&self.memory[pc + 3..pc + 6]),
            self.read_24_bits(&self.memory[pc + 6..pc + 9]),
        );

        self.memory[addr_b] = self.memory[addr_a];
        self.program_counter = addr_jump;
    }

    pub fn update_keyboard_state(&mut self, key_values: u16) {
        self.memory[0..2].copy_from_slice(&key_values.to_be_bytes());
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        self.program_counter = self.read_24_bits(&self.memory[2..5]);

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
