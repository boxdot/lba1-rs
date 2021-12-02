use crate::lib3d::func::cross_mult_32;
use crate::sdl_engine::{delay_ms, SdlEngine};

const PALETTE_WIDTH: usize = 768;

#[derive(Debug)]
pub struct Palette {
    pub data: [u8; PALETTE_WIDTH],
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            data: [0; PALETTE_WIDTH],
        }
    }
}

#[allow(clippy::identity_op)]
fn fade_pal(engine: &mut SdlEngine, r: u8, g: u8, b: u8, pal: &Palette, percent: u32) {
    let mut workpal = Palette::default();
    for n in 0..256 {
        workpal.data[n * 3 + 0] =
            cross_mult_32(r.into(), pal.data[n * 3 + 0].into(), 100, percent) as u8;
        workpal.data[n * 3 + 1] =
            cross_mult_32(g.into(), pal.data[n * 3 + 1].into(), 100, percent) as u8;
        workpal.data[n * 3 + 2] =
            cross_mult_32(b.into(), pal.data[n * 3 + 2].into(), 100, percent) as u8;
    }
    engine.palette(&workpal);
}

pub fn white_fade(engine: &mut SdlEngine) {
    let mut pal = Palette::default();
    for n in 0..255 {
        pal.data.fill(n);
        engine.palette(&pal);
        delay_ms(10);
    }
}

pub fn fade_white_to_pal(engine: &mut SdlEngine, pal: &Palette) {
    for n in 0..100 {
        fade_pal(engine, 255, 255, 255, pal, n);
        delay_ms(10);
    }
}
