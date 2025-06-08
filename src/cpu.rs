use crate::memory::Memory;
use crate::vm::{MEMORY_SIZE, PROGRAM_COUNTER_ADDR};
use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn tick(&mut self) {
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
}
