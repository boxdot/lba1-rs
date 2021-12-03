const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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
}
