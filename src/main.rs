use std::path::PathBuf;

use anyhow::{bail, Context as _};

use crate::ambiance::fade_to_black_pcx;
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
    let root = assets_path()?;

    let engine = SdlEngine::new().context("failed to init sdl engine")?;

    // TODO: read from setup.lst
    const VERSION_US: bool = true;

    let mut game = Game::new(root, engine);
    game.adeline_logo()?;

    fade_to_black_pcx(&mut game);

    // bumper
    if VERSION_US {
        ress_pict(&mut game, common::RESS_BUMPER_PCR)?;
    } else {
        ress_pict(&mut game, common::RESS_BUMPER2_PCR)?;
    };
    timer_pause(4);
    fade_to_black_pcx(&mut game);

    // logo EA
    ress_pict(&mut game, common::RESS_BUMPER_EA_PCR)?;
    timer_pause(2);
    fade_to_black_pcx(&mut game);

    // FLA intro
    play_anim_fla(&mut game, "dragon3")?;

    Ok(())
}

fn assets_path() -> anyhow::Result<PathBuf> {
    let cd_root = if let Some(path) = std::env::args().nth(1) {
        PathBuf::from(path)
    } else {
        let local_cd = PathBuf::from("cd");
        if !local_cd.exists() || !local_cd.is_dir() {
            bail!("please provide a path to the ADELINE CD");
        }
        local_cd
    };
    Ok(cd_root.join("lba"))
}
