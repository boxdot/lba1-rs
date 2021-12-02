use std::time::Duration;

use sdl2::pixels::{Color, Palette as SdlPalette, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::ambiance::Palette;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

pub struct SdlEngine {
    pub window_canvas: Canvas<Window>,
    pub event_pump: EventPump,
    buffer_surface: Surface<'static>,
    colors_buffer: Vec<Color>,
}

impl std::fmt::Debug for SdlEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SdlEngine").finish()
    }
}

impl SdlEngine {
    pub fn new() -> anyhow::Result<Self> {
        let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
        let video_subsystem = sdl_context.video().map_err(anyhow::Error::msg)?;

        let window = video_subsystem
            .window("Little Big Adventure", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()?;

        let mut window_canvas = window.into_canvas().present_vsync().build()?;

        window_canvas.set_draw_color(Color::BLACK);
        window_canvas.clear();
        window_canvas.present();

        let event_pump = sdl_context.event_pump().map_err(anyhow::Error::msg)?;

        Ok(Self {
            window_canvas,
            event_pump,
            buffer_surface: Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::Index8)
                .map_err(anyhow::Error::msg)?,
            colors_buffer: vec![Color::BLACK; 256],
        })
    }

    pub fn palette(&mut self, pal: &Palette) {
        self.colors_buffer.resize(pal.data.len() / 3, Color::BLACK);
        for (i, rgb) in pal.data.chunks_exact(3).enumerate() {
            self.colors_buffer[i] = Color::RGB(rgb[0], rgb[1], rgb[2]);
        }

        let sdl_pal = SdlPalette::with_colors(self.colors_buffer.as_slice()).unwrap();
        self.buffer_surface.set_palette(&sdl_pal).unwrap();

        let mut screen_surface = self
            .window_canvas
            .window()
            .surface(&self.event_pump)
            .unwrap();
        self.buffer_surface
            .blit(None, &mut screen_surface, None)
            .unwrap();
        screen_surface.finish().unwrap();
    }

    pub fn copy_from_buffer(&mut self, buf: &[u8]) {
        self.buffer_surface.with_lock_mut(|data| {
            data.copy_from_slice(buf);
        });
    }

    pub fn flip(&mut self) {
        let mut screen_surface = self
            .window_canvas
            .window()
            .surface(&self.event_pump)
            .unwrap();
        self.buffer_surface
            .blit(None, &mut screen_surface, None)
            .unwrap();
        screen_surface.finish().unwrap();
    }
}

pub fn delay_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
