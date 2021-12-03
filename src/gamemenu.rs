use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use crate::ambiance::fade_to_pal;
use crate::ambiance::fade_white_to_pal;
use crate::ambiance::set_black_pal;
use crate::ambiance::white_fade;
use crate::common;
use crate::global::Global;
use crate::hqr_ress::load_hqr;
use crate::screen::Screen;
use crate::sdl_engine::SdlEngine;

use anyhow::Context as _;

#[derive(Debug)]
pub struct Game {
    pub engine: SdlEngine,

    screen: Screen,
    log: Screen,

    pub global: Global,
}

impl Game {
    pub fn new(engine: SdlEngine) -> Self {
        Self {
            engine,

            screen: Default::default(),
            log: Default::default(),

            global: Default::default(),
        }
    }

    pub fn adeline_logo(&mut self) -> anyhow::Result<()> {
        load_hqr("RESS.HQR", &mut self.screen.data, common::RESS_LOGO_PCR)
            .context("failed to load logo pcr from RESS.HQR")?;
        self.screen.copy_to(&mut self.log);
        load_hqr(
            "RESS.HQR",
            &mut self.global.palette_pcx.data,
            common::RESS_LOGO_PAL,
        )
        .context("failed to load logo palette from RESS.HQR")?;
        white_fade(&mut self.engine);
        flip(self);
        fade_white_to_pal(&mut self.engine, &self.global.palette_pcx);
        Ok(())
    }
}

pub fn flip(game: &mut Game) {
    game.engine.copy_from_buffer(&game.log.data);
    game.engine.flip();
}

pub fn ress_pict(game: &mut Game, index: usize) -> anyhow::Result<()> {
    set_black_pal(game);
    load_hqr("RESS.HQR", &mut game.screen.data, index)?;
    game.screen.copy_to(&mut game.log);
    load_hqr("RESS.HQR", &mut game.global.palette_pcx.data, index + 1)?;
    flip(game);
    fade_to_pal(game, None);
    Ok(())
}

pub fn timer_pause(secs: u64) {
    let now = Instant::now();
    let dur = Duration::from_secs(secs);
    while now.elapsed() < dur {
        // TODO: cancel on key
        sleep(Duration::from_millis(10));
    }
}

pub fn timer_esc(secs: u64) {
    let now = Instant::now();
    let dur = Duration::from_secs(secs);
    while now.elapsed() < dur {
        // TODO: cancel on key
        sleep(Duration::from_millis(10));
    }
}
