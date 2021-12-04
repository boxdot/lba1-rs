use std::io;

use crate::ambiance::{fade_to_black, fade_to_pal};
use crate::common::RESS_FLA_PCX;
use crate::gamemenu::{flip, timer_esc, Game};
use crate::hqr_ress::load_hqrm;

pub fn play_anim_fla(game: &mut Game, name: &str) -> io::Result<()> {
    play_disk_fla(game, name)
}

fn play_disk_fla(game: &mut Game, name: &str) -> io::Result<()> {
    let txt = load_hqrm(game.root.join("ress.hqr"), RESS_FLA_PCX)?;

    let name = name.splitn(2, '.').next().unwrap();

    let txt = std::str::from_utf8(&txt).unwrap();

    let indexes = search_fla(name, txt);
    for index in indexes {
        match index {
            200 => {}
            201 => {}
            202 => {}
            _ => return Err(io::Error::new(io::ErrorKind::Unsupported, "todo")),
        }

        flip(game);
        fade_to_pal(game, None);

        timer_esc(4);

        // TODO: return on escape

        fade_to_black(game, None);
    }

    Ok(())
}

fn search_fla<'a>(name: &'a str, txt: &'a str) -> impl Iterator<Item = usize> + 'a {
    txt.lines()
        .filter_map(|l| {
            let mut parts = l.split_whitespace();
            let line_name = parts.next()?;
            if line_name.to_ascii_lowercase() == name.to_ascii_lowercase() {
                Some(parts.filter_map(|s| s.parse::<usize>().ok()))
            } else {
                None
            }
        })
        .flatten()
}
