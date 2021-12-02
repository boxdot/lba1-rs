use anyhow::Context as _;

use gamemenu::Game;
use sdl_engine::SdlEngine;

mod ambiance;
mod common;
mod gamemenu;
mod hqr_ress;
mod lib3d;
mod libsys;
mod screen;
mod sdl_engine;

fn main() -> anyhow::Result<()> {
    let engine = SdlEngine::new().context("failed to init sdl engine")?;

    let mut game = Game::new(engine);
    game.adeline_logo()?;

    Ok(())
}
