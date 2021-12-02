use crate::ambiance::fade_white_to_pal;
use crate::ambiance::white_fade;
use crate::ambiance::Palette;
use crate::common;
use crate::hqr_ress::load_hqr;
use crate::screen::Screen;
use crate::sdl_engine::SdlEngine;

use anyhow::Context as _;

#[derive(Debug)]
pub struct Game {
    pub engine: SdlEngine,
    screen: Screen,
    log: Screen,
    palette_pcx: Palette,
}

impl Game {
    pub fn new(engine: SdlEngine) -> Self {
        Self {
            engine,
            screen: Default::default(),
            log: Default::default(),
            palette_pcx: Default::default(),
        }
    }

    pub fn adeline_logo(&mut self) -> anyhow::Result<()> {
        load_hqr("RESS.HQR", &mut self.screen.data, common::RESS_LOGO_PCR)
            .context("failed to load logo pcr from RESS.HQR")?;
        self.screen.copy_to(&mut self.log);
        load_hqr(
            "RESS.HQR",
            &mut self.palette_pcx.data,
            common::RESS_LOGO_PAL,
        )
        .context("failed to load logo palette from RESS.HQR")?;
        white_fade(&mut self.engine);
        flip(self);
        fade_white_to_pal(&mut self.engine, &self.palette_pcx);
        Ok(())
    }
}

fn flip(game: &mut Game) {
    game.engine.copy_from_buffer(&game.log.data);
    game.engine.flip();
}
