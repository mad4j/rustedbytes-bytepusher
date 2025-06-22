use std::{cell::RefCell, rc::Rc};

use crate::{
    memory::Memory,
    vm::{SCREEN_BUFFER_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH},
};
use image::{ImageBuffer, Rgb};

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

    pub fn render_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let new_frame = self.get_screen_buffer();

        let mut screen = [0u32; SCREEN_BUFFER_SIZE];

        screen
            .iter_mut()
            .zip(new_frame.iter())
            .for_each(|(screen_pixel, &frame_pixel)| {
                *screen_pixel = self.palette[frame_pixel as usize];
            });

        self.window
            .borrow_mut()
            .update_with_buffer(&screen, SCREEN_WIDTH, SCREEN_HEIGHT)?;

        Ok(())
    }

    pub fn save_screen_png(&self, file_index: u32) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = self.get_screen_buffer();
        let mut img_buf = ImageBuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        for (i, pixel) in buffer.iter().enumerate() {
            let color = self.palette[*pixel as usize];
            let r = ((color >> 16) & 0xFF) as u8;
            let g = ((color >> 8) & 0xFF) as u8;
            let b = (color & 0xFF) as u8;
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;
            img_buf.put_pixel(x, y, Rgb([r, g, b]));
        }
        let filename = format!("screenshot_{:04}.png", file_index);
        img_buf.save(&filename)?;
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

    fn get_screen_buffer(&self) -> [u8; SCREEN_BUFFER_SIZE] {
        let mem = self.memory.borrow();
        let graphics_addr = (mem[self.memory_register_addr] as usize) << 16;
        let new_frame = &mem[graphics_addr..graphics_addr + SCREEN_BUFFER_SIZE];
        let mut arr = [0u8; SCREEN_BUFFER_SIZE];
        arr.copy_from_slice(new_frame);
        arr
    }
}
