use gamemenu::Game;

mod ambiance;
mod common;
mod gamemenu;
mod hqr_ress;
mod lib3d;
mod libsys;
mod screen;

fn main() -> anyhow::Result<()> {
    let mut game = Game::default();
    game.adeline_logo()?;
    Ok(())
}
