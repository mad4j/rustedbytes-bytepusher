const MEMORY_SIZE: usize = 16 * 1024 * 1024; // 16MB

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;

pub struct Cpu {
    program_counter: usize,
    palette: [u32; 256],
    pub memory: Vec<u8>,
    pub screen: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub sample_buffer: [u8; 256],
    pub keys: [bool; 16],
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
            palette,
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            sample_buffer: [0; 256],
            keys: [false; 16],
        }
    }
}

impl Cpu {
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[..rom.len()].copy_from_slice(rom);
    }

    pub fn render(&mut self, new_frame: [u8; 65536]) {
        // Copy the new frame into the screen buffer using the palette
        self.screen
            .iter_mut()
            .zip(new_frame.iter())
            .for_each(|(screen_pixel, &frame_pixel)| {
                *screen_pixel = self.palette[frame_pixel as usize];
            });
    }

    #[inline(always)]
    fn read_24_bits(&self, slice: &[u8]) -> usize {
        (u32::from_be_bytes([slice[0], slice[1], slice[2], 0]) >> 8) as usize
    }

    fn execute_instruction(&mut self) {
        // Read program counter
        let pc = self.program_counter;

        // Read addresses
        let (addr_a, addr_b, addr_jump) = (
            self.read_24_bits(&self.memory[pc..pc + 3]),
            self.read_24_bits(&self.memory[pc + 3..pc + 6]),
            self.read_24_bits(&self.memory[pc + 6..pc + 9]),
        );

        // Perform memory copy and update program counter
        self.memory[addr_b] = self.memory[addr_a];
        self.program_counter = addr_jump;
    }

    #[inline(always)]
    pub fn tick(&mut self) {
        // set key presses
        let mut key_values: u16 = 0;
        for (i, b) in self.keys.iter().enumerate() {
            let bit = if *b { 1 } else { 0 };
            key_values |= bit << i;
        }
        self.memory[0..2].copy_from_slice(&key_values.to_be_bytes());

        self.program_counter = self.read_24_bits(&self.memory[2..5]);

        let graphics_addr = (self.memory[5] as usize) << 16;
        let new_frame: [u8; 65536] = self.memory[graphics_addr..graphics_addr + 65536]
            .try_into()
            .unwrap();
        self.render(new_frame);

        let audio_addr = self.memory[6] as usize * 65536 + self.memory[7] as usize * 256;
        self.sample_buffer = self.memory[audio_addr..audio_addr + 256]
            .try_into()
            .expect("Unable to load audio sample from memory");
        //dbg!(self.sample_buffer);

        for _ in 0..65536 {
            self.execute_instruction();
        }
    }
}
