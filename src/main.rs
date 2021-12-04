use std::path::PathBuf;

use anyhow::{anyhow, bail, Context as _};

use crate::ambiance::fade_to_black;
use crate::gamemenu::{ress_pict, timer_pause, Game};
use crate::playfla::play_anim_fla;
use crate::sdl_engine::SdlEngine;

mod ambiance;
mod common;
mod gamemenu;
mod global;
mod hqr_ress;
mod lib3d;
mod libsys;
mod playfla;
mod screen;
mod sdl_engine;

fn main() -> anyhow::Result<()> {
    let cd_path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing obligatory path to ADELINE CD"))?;
    let root = PathBuf::from(cd_path).join("lba");
    if !root.exists() {
        bail!("path {} does not exist", root.display());
    }

    let engine = SdlEngine::new().context("failed to init sdl engine")?;

    // TODO: read from setup.lst
    const VERSION_US: bool = true;

    let mut game = Game::new(root, engine);
    game.adeline_logo()?;

    fade_to_black(&mut game, None);

    // bumper
    if VERSION_US {
        ress_pict(&mut game, common::RESS_BUMPER_PCR)?;
    } else {
        ress_pict(&mut game, common::RESS_BUMPER2_PCR)?;
    };
    timer_pause(4);
    fade_to_black(&mut game, None);

    // logo EA
    ress_pict(&mut game, common::RESS_BUMPER_EA_PCR)?;
    timer_pause(2);
    fade_to_black(&mut game, None);

    // FLA intro
    play_anim_fla(&mut game, "dragon3")?;

    Ok(())
}
