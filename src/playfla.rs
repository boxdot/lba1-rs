use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::Context;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::ambiance::{
    fade_to_black, fade_to_black_pcx, fade_to_pal, fade_to_pal_pcx, set_black_pal, Palette,
};
use crate::common::RESS_FLA_PCX;
use crate::gamemenu::{flip, timer_esc, Game};
use crate::hqr_ress::load_hqrm;

const FLA_FROM_CD: bool = true;
const FLA_DIR: &str = "fla";
const FLA_EXT: &str = "fla";

const VERSION: &str = "V1.3";

pub fn play_anim_fla(game: &mut Game, name: &str) -> anyhow::Result<()> {
    if !FLA_FROM_CD {
        play_disk_fla(game, name)?;
        return Ok(());
    }

    // StopMusicCD

    let path = game.root.join(FLA_DIR).join(name).with_extension(FLA_EXT);
    let mut reader = BufReader::new(
        File::open(&path).context(format!("failed to open fla movie at {}", path.display()))?,
    );

    init_fla(&mut reader, &mut game.fla)?;

    if game.fla.header.version == VERSION {
        // ExtInitMcga
        set_black_pal(game);
        // Clear
        // Flip
        // Clear

        game.fla.flag_first = true;
        let frame_duration = Duration::from_millis(1000 / game.fla.header.cadence_animation as u64);

        for _ in 0..game.fla.header.num_frames {
            // exit on ESC
            let now = Instant::now();

            draw_next_frame_fla(game, &mut reader)?;

            flip(game);
            management_palette(game);

            sleep(frame_duration.saturating_sub(now.elapsed()));
        }
    }

    Ok(())
}

fn management_palette(game: &mut Game) {
    if game.fla.flag_first {
        fade_to_pal(
            &mut game.engine,
            &game.fla.palette,
            &mut game.global.flag_black_pal,
        );
        game.fla.flag_first = false;
        game.fla.color_len = 0;
    }

    if game.fla.color_len != 0 {
        game.engine.palette(&game.fla.palette);
        game.fla.color_len = 0;
    }
}

#[derive(Debug, Default)]
pub struct Fla {
    palette: Palette,
    header: HeaderFla,
    header_block: HeaderFlaBlock,
    header_sample: FlaSample,
    header_balance: FlaBalance,
    header_sample_stop: FlaSampleStop,
    header_info: FlaInfo,

    color_start: usize,
    color_len: usize,
    flag_first: bool,
}

fn draw_next_frame_fla(game: &mut Game, reader: impl Read) -> io::Result<()> {
    let len = load_next_frame_fla(game, reader)?;

    let fla = &mut game.fla;
    let mut buffer = &game.screen.data[0..len];

    let log = &mut game.log.data;

    let block_len = fla.header_block.block_len;

    for _ in 0..block_len {
        let header_type = FlaType::from_reader(&mut buffer)?;

        let mut data = buffer;

        match header_type.typ {
            FlaTypeEnum::Palette => {
                let header_palette = FlaPalette::from_reader(&mut data)?;
                let start = header_palette.color_start as usize;
                let len = header_palette.num_colors as usize;
                fla.palette.data[start..start + len * 3].copy_from_slice(&data[0..len * 3]);
                fla.color_start = start;
                fla.color_len = len;
            }
            FlaTypeEnum::Info => {
                fla.header_info = FlaInfo::from_reader(&mut data)?;
                match fla.header_info.info {
                    1 => {
                        // TODO: play flute
                    }
                    2 => {
                        fade_to_black(
                            &mut game.engine,
                            &fla.palette,
                            &mut game.global.flag_black_pal,
                        );
                    }
                    3 => {
                        fla.flag_first = true;
                    }
                    4 => {
                        // fade music midi
                    }
                    _ => return Err(io::Error::new(io::ErrorKind::Other, "invalid fla info")),
                }
            }
            FlaTypeEnum::Sample => {
                fla.header_sample = FlaSample::from_reader(&mut data)?;
                // TODO: play sample
            }
            FlaTypeEnum::SampleBalance => {
                fla.header_balance = FlaBalance::from_reader(&mut data)?;
                // TODO: change balance
            }
            FlaTypeEnum::SampleStop => {
                fla.header_sample_stop = FlaSampleStop::from_reader(&mut data)?;
                // TODO: stop sample
            }
            FlaTypeEnum::Lc => update_frame(log, data, 320),
            FlaTypeEnum::Black => black_frame(log),
            FlaTypeEnum::Brown => draw_frame(log, data, 320, fla.header.resolution_y as usize),
            FlaTypeEnum::Copy => copy_frame(log, &data[0..320 * 200]),
        }

        buffer = &buffer[header_type.offset_next_block as usize..];
    }

    Ok(())
}

fn load_next_frame_fla(game: &mut Game, mut reader: impl Read) -> io::Result<usize> {
    game.fla.header_block = HeaderFlaBlock::from_reader(&mut reader)?;

    let offset = game.fla.header_block.next_frame_offset as usize;
    reader.read_exact(&mut game.screen.data[0..offset])?;

    Ok(offset)
}

fn init_fla(mut reader: impl Read, fla: &mut Fla) -> io::Result<()> {
    // TODO: load samples

    fla.header = HeaderFla::from_reader(&mut reader)?;
    // Note: no need to store it, only to skip the memory
    let header_sample_list_fla = FlaSampleList::from_reader(&mut reader)?;

    for _ in 0..header_sample_list_fla.num_samples {
        let _sample_num = &mut reader.read_u16::<LittleEndian>()?;
        let _nb_fois_joue = reader.read_u16::<LittleEndian>()?; // ?

        // TODO: preload samples
    }

    Ok(())
}

fn play_disk_fla(game: &mut Game, name: &str) -> io::Result<()> {
    let txt = load_hqrm(game.root.join("ress.hqr"), RESS_FLA_PCX)?;

    let (name, _) = name.split_once('.').unwrap();

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
        fade_to_pal_pcx(game);

        timer_esc(4);

        // TODO: return on escape

        fade_to_black_pcx(game);
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

#[derive(Debug, Default)]
struct HeaderFla {
    version: String,
    num_frames: u32,
    cadence_animation: u8,
    _resolution_x: u16,
    resolution_y: u16,
}

impl HeaderFla {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let mut version = [0; 5];
        reader.read_exact(&mut version)?;
        let version = CStr::from_bytes_with_nul(&version)
            .ok()
            .and_then(|s| CString::from(s).into_string().ok())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid version string"))?;
        reader.read_u8()?; // 16 bit alignment
        let num_frames = reader.read_u32::<LittleEndian>()?;
        let cadence_animation = reader.read_u8()?;
        reader.read_u8()?; // 16 bit alignment
        Ok(Self {
            version,
            num_frames,
            cadence_animation,
            _resolution_x: reader.read_u16::<LittleEndian>()?,
            resolution_y: reader.read_u16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
struct HeaderFlaBlock {
    block_len: u8,
    next_frame_offset: u32,
}

impl HeaderFlaBlock {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let block_len = reader.read_u8()?;
        reader.read_u8()?; // 16 bit alignment
        Ok(Self {
            block_len,
            next_frame_offset: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct FlaSample {
    n: i16,
    displacement: i16,
    repeat: i16,
    balance: u8,
    volume_g: u8,
    volume_d: u8,
}

impl FlaSample {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let n = reader.read_i16::<LittleEndian>()?;
        let displacement = reader.read_i16::<LittleEndian>()?;
        let repeat = reader.read_i16::<LittleEndian>()?;
        let balance = reader.read_u8()?;
        let volume_g = reader.read_u8()?;
        let volume_d = reader.read_u8()?;
        reader.read_u8()?; // 16 bit alignment
        Ok(Self {
            n,
            displacement,
            repeat,
            balance,
            volume_g,
            volume_d,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct FlaBalance {
    n: i16,
    offset: u8,
    balance: i16,
    volume_g: u8,
    volume_d: u8,
}

impl FlaBalance {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        Ok(Self {
            n: reader.read_i16::<LittleEndian>()?,
            offset: reader.read_u8()?,
            balance: reader.read_i16::<LittleEndian>()?,
            volume_g: reader.read_u8()?,
            volume_d: reader.read_u8()?,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct FlaSampleStop {
    n: u16,
}

impl FlaSampleStop {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        Ok(Self {
            n: reader.read_u16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
struct FlaInfo {
    info: i16,
}

impl FlaInfo {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        Ok(Self {
            info: reader.read_i16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
struct FlaSampleList {
    num_samples: i16,
    _offset_frame_one: i16,
}

impl FlaSampleList {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        Ok(Self {
            num_samples: reader.read_i16::<LittleEndian>()?,
            _offset_frame_one: reader.read_i16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
struct FlaType {
    typ: FlaTypeEnum,
    offset_next_block: u16,
}

impl FlaType {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let byte = reader.read_u8()?;
        let typ = FlaTypeEnum::from_u8(byte).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("invalid fla type enum {}", byte),
            )
        })?;
        reader.read_u8()?; // 16 bit alignment
        Ok(Self {
            typ,
            offset_next_block: reader.read_u16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
struct FlaPalette {
    num_colors: u16,
    color_start: u16,
}

impl FlaPalette {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        Ok(Self {
            num_colors: reader.read_u16::<LittleEndian>()?,
            color_start: reader.read_u16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
enum FlaTypeEnum {
    Palette,
    Info,
    Sample,
    SampleBalance,
    SampleStop,
    Lc,
    Black,
    Brown,
    Copy,
}

impl Default for FlaTypeEnum {
    fn default() -> Self {
        FlaTypeEnum::Palette
    }
}

impl FlaTypeEnum {
    fn from_u8(byte: u8) -> Option<Self> {
        Some(match byte {
            1 => Self::Palette,
            2 => Self::Info,
            3 => Self::Sample,
            4 => Self::SampleBalance,
            5 => Self::SampleStop,
            6 => Self::Lc,
            7 => Self::Black,
            8 => Self::Brown,
            9 | 16 => Self::Copy,
            _ => return None,
        })
    }
}

fn black_frame(dest: &mut [u8]) {
    for row in 0..200 {
        let start = row * 320;
        dest[start..start + 320].fill(0);
    }
}

fn copy_frame(dest: &mut [u8], data: &[u8]) {
    for (row, row_data) in data.chunks_exact(320).enumerate() {
        let start = row * 320;
        dest[start..start + 320].copy_from_slice(row_data);
    }
}

fn draw_frame(dest: &mut [u8], data: &[u8], width: usize, height: usize) {
    let mut src_idx = 0;

    for y in 0..height {
        let mut dst_idx = y * width;

        let num_blocks = data[src_idx];
        src_idx += 1;

        for _ in 0..num_blocks {
            let flag = data[src_idx] as i8;
            src_idx += 1;

            let len = flag.abs() as usize;

            if flag < 0 {
                dest[dst_idx..dst_idx + len].copy_from_slice(&data[src_idx..src_idx + len]);
                src_idx += len;
            } else {
                let color_index = data[src_idx];
                src_idx += 1;
                dest[dst_idx..dst_idx + len].fill(color_index);
            }
            dst_idx += len;
        }
    }
}

fn update_frame(dest: &mut [u8], data: &[u8], width: usize) {
    let mut src_idx = 0;

    let delta = u16::from_le_bytes([data[src_idx], data[src_idx + 1]]) as usize * width;
    src_idx += 2;

    let height = u16::from_le_bytes([data[src_idx], data[src_idx + 1]]) as usize;
    src_idx += 2;

    for y in 0..height {
        let mut dst_idx = delta + y * width;

        let num_blocks = data[src_idx];
        src_idx += 1;

        for _ in 0..num_blocks {
            dst_idx += data[src_idx] as usize;
            src_idx += 1;

            let flag = data[src_idx] as i8;
            src_idx += 1;

            let len = flag.abs() as usize;

            if flag > 0 {
                dest[dst_idx..dst_idx + len].copy_from_slice(&data[src_idx..src_idx + len]);
                src_idx += len;
            } else {
                let color_index = data[src_idx];
                src_idx += 1;
                dest[dst_idx..dst_idx + len].fill(color_index);
            }
            dst_idx += len;
        }
    }
}
