const MEMORY_SIZE: usize = 16_777_216;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;

//use bit_struct::u24;

pub struct Emulator {
    program_counter: usize,
    palette: [u32; 256],
    pub memory: Vec<u8>,
    pub screen: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub sample_buffer: [u8; 256],
    pub keys: [bool; 16],
}

impl Default for Emulator {
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

impl Emulator {
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory[0..rom.len()].copy_from_slice(rom.as_slice());
    }

    pub fn render(&mut self, new_frame: [u8; 65536]) {
        /*self.screen = new_frame
        .iter()
        .map(|val| self.palette[*val as usize])
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap();*/
        for (idx, val) in new_frame.iter().enumerate() {
            //dbg!(val, self.palette[*val as usize]);
            self.screen[idx] = self.palette[*val as usize];
        }
    }

    fn read_24_bits(&self, slice: &[u8]) -> usize {
        ((slice[0] as usize) << 16) | ((slice[1] as usize) << 8) | slice[2] as usize
    }

    /*fn read_opcode(&self) -> (usize, usize, usize) {
        let pc = self.program_counter;
        let addr_a = u24::from_be_bytes(self.memory[pc..pc+3].try_into().expect("Unable to read first address in opcode"));
        let addr_b = u24::from_be_bytes(self.memory[pc+3..pc+6].try_into().expect("Unable to read second address in opcode"));
        let addr_jump = u24::from_be_bytes(self.memory[pc+6..pc+9].try_into().expect("Unable to read jump address in opcode"));
        (addr_a.value() as usize, addr_b.value() as usize, addr_jump.value() as usize)
    }*/

    fn execute_instruction(&mut self) {
        let pc = self.program_counter;

        let addr_a = self.read_24_bits(&self.memory[pc..pc + 3]);
        let addr_b = self.read_24_bits(&self.memory[pc + 3..pc + 6]);
        //println!("{:?}", opcode);

        self.memory[addr_b] = self.memory[addr_a];

        let addr_jump = self.read_24_bits(&self.memory[pc + 6..pc + 9]);
        self.program_counter = addr_jump;
    }

    pub fn tick(&mut self) {
        // set key presses
        let mut key_values: u16 = 0;
        for (i, b) in self.keys.iter().enumerate() {
            let bit = if *b { 1 } else { 0 };
            key_values |= bit << i;
        }
        self.memory[0..2].copy_from_slice(&key_values.to_be_bytes());

        self.program_counter = self.read_24_bits(&self.memory[2..5]);

        let graphics_addr = (self.memory[5] as usize) * 65536;
        let new_frame: [u8; 65536] = self.memory[graphics_addr..graphics_addr + 65536]
            .try_into()
            .expect("Unable to load frame from memory");
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
