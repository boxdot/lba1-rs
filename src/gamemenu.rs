use crate::ambiance::Palette;
use crate::common;
use crate::hqr_ress::load_hqr;
use crate::screen::Screen;

use anyhow::Context as _;

#[derive(Debug, Default)]
pub struct Game {
    screen: Screen,
    log: Screen,
    palette_pcx: Palette,
}

impl Game {
    pub fn adeline_logo(&mut self) -> anyhow::Result<()> {
        load_hqr("RESS.HQR", &mut self.screen.data, common::RESS_LOGO_PCR)
            .context("failed to load logo pcr from RESS.HQR")?;
        self.screen.copy_to(&mut self.log);
        load_hqr(
            "RESS.HQR",
            &mut self.palette_pcx.data,
            common::RESS_LOGO_PAL,
        )
        .context("failed to load logo palette RESS.HQR")?;
        // white_fade();
        // flip();
        // fade_white_to_pal(self.palette_pcx);
        Ok(())
    }
}
