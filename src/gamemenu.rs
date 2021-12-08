use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::ambiance::{fade_to_pal_pcx, fade_white_to_pal, set_black_pal, white_fade};
use crate::common;
use crate::global::Global;
use crate::hqr_ress::load_hqr;
use crate::playfla::Fla;
use crate::screen::Screen;
use crate::sdl_engine::SdlEngine;

use anyhow::Context as _;

#[derive(Debug)]
pub struct Game {
    pub engine: SdlEngine,

    pub root: PathBuf,

    pub screen: Screen,
    pub log: Screen,

    pub global: Global,

    pub fla: Fla,
}

impl Game {
    pub fn new(root: impl Into<PathBuf>, engine: SdlEngine) -> Self {
        Self {
            engine,

            root: root.into(),

            screen: Default::default(),
            log: Default::default(),

            global: Default::default(),

            fla: Default::default(),
        }
    }

    pub fn adeline_logo(&mut self) -> anyhow::Result<()> {
        load_hqr(
            self.root.join("ress.hqr"),
            &mut self.screen.data,
            common::RESS_LOGO_PCR,
        )
        .context("failed to load logo pcr from ress.hqr")?;
        self.screen.copy_to(&mut self.log);
        load_hqr(
            self.root.join("ress.hqr"),
            &mut self.global.palette_pcx.data,
            common::RESS_LOGO_PAL,
        )
        .context("failed to load logo palette from ress.hqr")?;
        white_fade(&mut self.engine);
        flip(self);
        fade_white_to_pal(&mut self.engine, &self.global.palette_pcx);
        Ok(())
    }
}

pub fn clear(game: &mut Game) {
    game.log.data.fill(0);
}

pub fn flip(game: &mut Game) {
    game.engine.copy_from_buffer(&game.log.data);
    game.engine.flip();
}

pub fn ress_pict(game: &mut Game, index: usize) -> anyhow::Result<()> {
    set_black_pal(game);
    load_hqr(game.root.join("ress.hqr"), &mut game.screen.data, index)?;
    game.screen.copy_to(&mut game.log);
    load_hqr(
        game.root.join("ress.hqr"),
        &mut game.global.palette_pcx.data,
        index + 1,
    )?;
    flip(game);
    fade_to_pal_pcx(game);
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
