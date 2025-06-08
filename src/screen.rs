use std::{cell::RefCell, rc::Rc};

use crate::{cpu, memory::Memory};

pub struct ScreenHandler {
    pub palette: [u32; 256],
    pub memory_register_addr: usize,
    pub memory: Rc<RefCell<Memory>>,
    pub window: Rc<RefCell<minifb::Window>>,
}

impl ScreenHandler {
    pub fn new(
        memory_register_addr: usize,
        memory: Rc<RefCell<Memory>>,
        window: Rc<RefCell<minifb::Window>>,
    ) -> Self {
        let palette = Self::init_palette();

        Self {
            palette,
            memory_register_addr,
            memory,
            window,
        }
    }

    pub fn render_frame(&mut self)  -> Result<(), Box<dyn std::error::Error>> {
        let new_frame = self.get_screen_buffer();

        let mut screen = [0u32; cpu::SCREEN_BUFFER_SIZE];

        screen
            .iter_mut()
            .zip(new_frame.iter())
            .for_each(|(screen_pixel, &frame_pixel)| {
                *screen_pixel = self.palette[frame_pixel as usize];
            });

        self.window.borrow_mut().update_with_buffer(
            &screen,
            cpu::SCREEN_WIDTH,
            cpu::SCREEN_HEIGHT,
        )?;

        Ok(())
    }


    fn init_palette() -> [u32; 256] {
        let mut palette: [u32; 256] = [0; 256];
        for (idx, val) in palette.iter_mut().enumerate() {
            if idx >= 216 {
                break;
            }
            *val = ((idx as u32 / 36 * 0x33) << 16)
                | ((idx as u32 / 6 % 6 * 0x33) << 8)
                | (idx as u32 % 6 * 0x33);
        }
        palette
    }

    fn get_screen_buffer(&self) -> [u8; cpu::SCREEN_BUFFER_SIZE] {
        let mem = self.memory.borrow();
        let graphics_addr = (mem[self.memory_register_addr] as usize) << 16;
        let new_frame = &mem[graphics_addr..graphics_addr + cpu::SCREEN_BUFFER_SIZE];
        let mut arr = [0u8; cpu::SCREEN_BUFFER_SIZE];
        arr.copy_from_slice(new_frame);
        arr
    }
}
