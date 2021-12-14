pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;

const CLIP_X_MIN: u32 = 0;
const CLIP_Y_MIN: u32 = 0;
const CLIP_X_MAX: u32 = 639;
const CLIP_Y_MAX: u32 = 479;

#[derive(Debug)]
pub struct Screen {
    pub data: [u8; WIDTH * HEIGHT],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            data: [0; WIDTH * HEIGHT],
        }
    }
}

impl Screen {
    pub fn copy_to(&self, dst: &mut Screen) {
        dst.data.copy_from_slice(&self.data);
    }

    pub fn draw_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: u8) {
        todo!();
    }

    pub fn draw_box(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: u8) {}
}
