use crate::ambiance::Palette;

#[derive(Debug, Default)]
pub struct Global {
    pub palette_pcx: Palette,

    pub flag_black_pal: bool,
}
