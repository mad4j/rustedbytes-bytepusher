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
        // needed to reduce borrow checker issues
        let pc = self.program_counter;
        let memory = &mut *self.memory.borrow_mut();

        // Fetch addresses of instruction operands
        let (addr_a, addr_b) = (
            memory.read_24_bits(pc),
            memory.read_24_bits(pc + 3),
        );

        // Execute the instruction (only one instruction for now)
        memory[addr_b] = memory[addr_a];

        // Move to the next instruction
        self.program_counter = memory.read_24_bits(pc + 6);
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
