

pub struct ScreenHandler {
    pub screen: [u32; 65536],
    pub palette: [u32; 256],
}

impl ScreenHandler {
    pub fn new() -> Self {
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
            screen: [0; 65536],
            palette,
        }
    }

    pub fn render(&mut self, new_frame: &[u8; 65536]) {
        self.screen
            .iter_mut()
            .zip(new_frame.iter())
            .for_each(|(screen_pixel, &frame_pixel)| {
                *screen_pixel = self.palette[frame_pixel as usize];
            });
    }

    pub fn get_screen(&self) -> &[u32; 65536] {
        &self.screen
    }
}