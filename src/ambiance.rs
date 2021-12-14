use std::io;

use crate::gamemenu::Game;
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

impl TryFrom<Vec<u8>> for Palette {
    type Error = io::Error;

    fn try_from(data: Vec<u8>) -> io::Result<Self> {
        Ok(Palette {
            data: data
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "unexpected data size"))?,
        })
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

pub fn fade_to_black(engine: &mut SdlEngine, pal: &Palette, flag_black_pal: &mut bool) {
    if !*flag_black_pal {
        for n in (0..=100).rev().step_by(2) {
            fade_pal(engine, 0, 0, 0, pal, n);
            delay_ms(10);
        }
    }
    *flag_black_pal = true;
}

pub fn fade_to_black_pcx(game: &mut Game) {
    fade_to_black(
        &mut game.engine,
        &game.global.palette_pcx,
        &mut game.global.flag_black_pal,
    );
}

pub fn set_black_pal(game: &mut Game) {
    game.engine.set_black_pal();
    game.global.flag_black_pal = true;
}

pub fn fade_to_pal(engine: &mut SdlEngine, pal: &Palette, flag_black_pal: &mut bool) {
    for n in (0..100).step_by(2) {
        fade_pal(engine, 0, 0, 0, pal, n);
        delay_ms(10);
    }
    *flag_black_pal = false;
}

pub fn fade_to_pal_pcx(game: &mut Game) {
    fade_to_pal(
        &mut game.engine,
        &game.global.palette_pcx,
        &mut game.global.flag_black_pal,
    );
}
