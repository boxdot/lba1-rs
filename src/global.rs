use crate::ambiance::Palette;

#[derive(Debug)]
pub struct Global {
    pub palette_pcx: Palette,
    pub palette: Palette,

    pub flag_black_pal: bool,

    pub buffer_speak: [u8; 256 * 1024 + 34],

    pub sample_volume: u32,
    pub music_volume: u32,
    pub cd_volume: u32,
    pub line_volume: u32,
    pub master_volume: u32,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            palette_pcx: Default::default(),
            palette: Default::default(),
            flag_black_pal: Default::default(),
            buffer_speak: [0; 256 * 1024 + 34],
            sample_volume: 0,
            music_volume: 0,
            cd_volume: 0,
            line_volume: 0,
            master_volume: 0,
        }
    }
}
