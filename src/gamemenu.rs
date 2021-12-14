use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::ambiance::{fade_to_pal_pcx, fade_white_to_pal, set_black_pal, white_fade};
use crate::common;
use crate::global::Global;
use crate::hqr_ress::load_hqr;
use crate::lib3d::func::cross_mult_32;
use crate::message::Message;
use crate::playfla::Fla;
use crate::screen::Screen;
use crate::sdl_engine::SdlEngine;

use anyhow::Context as _;

const DEFAULT_HEIGHT: usize = 50;
const MENU_SPACE: usize = 6;
const MENU_SIZE: usize = 550;
const COLOR_SELECT_MENU: u8 = 68;

#[derive(Debug)]
pub struct Game {
    pub engine: SdlEngine,

    pub root: PathBuf,

    pub screen: Screen,
    pub log: Screen,

    pub global: Global,

    pub fla: Fla,
    pub message: Message,
}

const GAME_MAIN_MENU: &[usize] = &[
    0,   // selected
    4,   // num entries
    200, // y center
    0,   // .dia num
    0, 20, // start a new game game
    0, 21, // continue game
    0, 23, // options
    0, 22, // quit
];

impl Game {
    pub fn new(root: impl Into<PathBuf>, engine: SdlEngine) -> Self {
        let root = root.into();
        Self {
            engine,

            root: root.clone(),

            screen: Default::default(),
            log: Default::default(),

            global: Default::default(),

            fla: Default::default(),
            message: Message::new(root),
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

    pub fn main_game_menu(mut self) -> anyhow::Result<()> {
        // stop sample

        self.screen.copy_to(&mut self.log);

        // loop {
        self.message.init_dial(0)?;

        // playcdtrack or playmidifile
        // hq_stopsample

        // self.get_multi_text(49, )

        self.do_game_menu(GAME_MAIN_MENU)?;

        // break;
        // }

        Ok(())
    }

    pub fn do_game_menu(&mut self, menu: &[usize]) -> anyhow::Result<()> {
        load_hqr(
            self.root.join("ress.hqr"),
            &mut self.global.buffer_speak,
            common::RESS_INIT_PLASMA,
        )?;

        let selected = menu[0];
        let num_entries = menu[1];

        // loop {
        // if menu == game main menu
        self.draw_game_menu(menu, false)?;
        // }

        Ok(())
    }

    // justone???
    pub fn draw_game_menu(&mut self, menu: &[usize], justone: bool) -> anyhow::Result<()> {
        let selected = menu[0];
        let num_entries = menu[1];
        let mut y = menu[2];

        if y == 0 {
            y = DEFAULT_HEIGHT / 2 + 10;
        } else {
            y -= (DEFAULT_HEIGHT * num_entries + (num_entries - 1) * MENU_SPACE) / 2;
        }

        for n in 0..num_entries {
            let typ = menu[5 + n];
            let num = menu[5 + n + 1];

            if justone {
                if n == selected {
                    self.draw_one_choice(320, y, typ, num, true);
                }
            } else {
                self.draw_one_choice(320, y, typ, num, n == selected);
            }

            y += DEFAULT_HEIGHT + MENU_SPACE;
        }

        Ok(())
    }

    fn draw_one_choice(&mut self, x: usize, y: usize, typ: usize, num: usize, select: bool) {
        let x0: u32 = (x - MENU_SIZE / 2) as u32;
        let x1: u32 = (x + MENU_SPACE / 2) as u32;
        let mut x2: u32 = 0;

        let y0: u32 = (y - DEFAULT_HEIGHT / 2) as u32;
        let y1: u32 = (y + DEFAULT_HEIGHT / 2) as u32;

        if select {
            match typ {
                1 => {
                    // music volume
                    x2 = cross_mult_32(x0, x1, 255, self.global.music_volume);
                    self.draw_fire(x0, y0, x2, y1, 91 & 0xF0);
                    self.draw_box(x2, y0, x1, y1, COLOR_SELECT_MENU);
                }
                2 => {
                    // sample volume
                    x2 = cross_mult_32(x0, x1, 255, self.global.sample_volume);
                    self.draw_fire(x0, y0, x2, y1, 91 & 0xF0);
                    self.draw_box(x2, y0, x1, y1, COLOR_SELECT_MENU);
                }
                3 => {
                    // cd volume
                    x2 = cross_mult_32(x0, x1, 255, self.global.cd_volume);
                    self.draw_fire(x0, y0, x2, y1, 91 & 0xF0);
                    self.draw_box(x2, y0, x1, y1, COLOR_SELECT_MENU);
                }
                4 => {
                    // line volume
                    x2 = cross_mult_32(x0, x1, 255, self.global.line_volume);
                    self.draw_fire(x0, y0, x2, y1, 91 & 0xF0);
                    self.draw_box(x2, y0, x1, y1, COLOR_SELECT_MENU);
                }
                5 => {
                    // master volume
                    x2 = cross_mult_32(x0, x1, 255, self.global.master_volume);
                    self.draw_fire(x0, y0, x2, y1, 91 & 0xF0);
                    self.draw_box(x2, y0, x1, y1, COLOR_SELECT_MENU);
                }
                _ => {
                    self.draw_fire(x0, y0, x1, y1, COLOR_SELECT_MENU & 0xF0);
                }
            }

            if (1..6).contains(&typ) {
                // TODO: play some sound
            }
        } else {
            copy_block(
                x0,
                y0,
                x1,
                y1,
                &self.screen.data,
                x0,
                y0,
                &mut self.log.data,
            );
            shade_box(x0, y0, x1, y1, 4);
        }

        self.draw_frame(x0, y0, x1, y1);

        // text
        // TODO

        // flip
        self.engine.copy_block_phys(&self.log.data, x0, y0, x1, y1);
    }

    fn draw_fire(&self, x0: u32, y0: u32, x2: u32, y1: u32, arg: u8) {
        todo!()
    }

    fn draw_box(&self, x2: u32, y0: u32, x1: u32, y1: u32, color: u8) {
        todo!()
    }

    fn draw_frame(&mut self, x0: u32, y0: u32, x1: u32, y1: u32) {
        self.log.draw_line(x0, y0, x1, y0, 79);
        self.log.draw_line(x0, y0, x0, y1, 79);
        self.log.draw_line(x1, y0 + 1, x1, y1, 73);
        self.log.draw_line(x0 + 1, y1, x1, y1, 73);
    }
}

fn shade_box(x0: u32, y0: u32, x1: u32, y1: u32, arg: i32) {
    todo!()
}

fn copy_block(x0_1: u32, y0_1: u32, x1: u32, y1: u32, src: &[u8], xd: u32, yd: u32, dst: &[u8]) {
    todo!()
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
