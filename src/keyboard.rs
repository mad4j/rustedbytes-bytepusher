use minifb::{Key, KeyRepeat, Window};
use std::cell::RefCell;
use std::rc::Rc;

use crate::memory::Memory;

pub struct KeyboardHandler {
    keyboard_state: u16,
    memory: Option<Rc<RefCell<Memory>>>,
    memory_regiter_addr: usize,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        Self {
            keyboard_state: 0,
            memory: None,
            memory_regiter_addr: 0,
        }
    }

    pub fn attach_memory(&mut self, memory: Rc<RefCell<Memory>>, memory_register_addr: usize) {
        self.memory = Some(memory);
        self.memory_regiter_addr = memory_register_addr;
    }

    pub fn handle_events(&mut self, window: &Window) {
        for key in window.get_keys_pressed(KeyRepeat::No) {
            if let Some(hex) = Self::key_to_hex(key) {
                self.keyboard_state |= 1 << hex;
            }
        }

        for key in window.get_keys_released() {
            if let Some(hex) = Self::key_to_hex(key) {
                self.keyboard_state &= !(1 << hex);
            }
        }

        if let Some(ref memory) = self.memory {
            memory
                .borrow_mut()
                .write_16_bits(self.memory_regiter_addr, self.keyboard_state);
        }
    }

    pub fn key_to_hex(key: Key) -> Option<u8> {
        match key {
            Key::Key1 => Some(0x1),
            Key::Key2 => Some(0x2),
            Key::Key3 => Some(0x3),
            Key::Key4 => Some(0xC),
            Key::Q => Some(0x4),
            Key::W => Some(0x5),
            Key::E => Some(0x6),
            Key::R => Some(0xD),
            Key::A => Some(0x7),
            Key::S => Some(0x8),
            Key::D => Some(0x9),
            Key::F => Some(0xE),
            Key::Z => Some(0xA),
            Key::X => Some(0x0),
            Key::C => Some(0xB),
            Key::V => Some(0xF),
            _ => None,
        }
    }
}
