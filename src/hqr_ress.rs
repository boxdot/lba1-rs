use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::libsys::decompress_lzs;

pub fn load_hqr(path: impl AsRef<Path>, buffer: &mut [u8], index: usize) -> io::Result<usize> {
    let mut file = BufReader::new(File::open(path.as_ref())?);

    let num_blocks = file.read_u32::<LittleEndian>()? as usize / 4;
    if num_blocks <= index {
        return Err(io::Error::new(io::ErrorKind::Other, "out of bounds"));
    }

    file.seek(SeekFrom::Start(index as u64 * 4))?;
    let seek_index = file.read_u32::<LittleEndian>()?;

    file.seek(SeekFrom::Start(seek_index.into()))?;
    let header = Header::from_reader(&mut file)?;

    if buffer.len() < header.size_file {
        return Err(io::Error::new(io::ErrorKind::Other, "buffer too small"));
    }

    match header.compress_method {
        CompressMethod::Stored => {
            file.read_exact(&mut buffer[0..header.size_file])?;
        }
        CompressMethod::Lzs => {
            // Note: we don't use the extra decompression margin from the buffer, how it is done
            // in the original source code.
            let mut compressed_buffer = vec![0; header.compressed_size_file];
            file.read_exact(&mut compressed_buffer)?;
            decompress_lzs(&compressed_buffer, &mut buffer[0..header.size_file]);
        }
    }

    Ok(header.size_file)
}

#[derive(Debug)]
struct Header {
    size_file: usize,
    compressed_size_file: usize,
    compress_method: CompressMethod,
}

impl Header {
    fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        Ok(Self {
            size_file: reader.read_u32::<LittleEndian>()? as usize,
            compressed_size_file: reader.read_u32::<LittleEndian>()? as usize,
            compress_method: CompressMethod::from_int(reader.read_u16::<LittleEndian>()?)?,
        })
    }
}

#[derive(Debug)]
enum CompressMethod {
    Stored,
    Lzs,
}

impl CompressMethod {
    fn from_int(value: u16) -> io::Result<Self> {
        match value {
            0 => Ok(Self::Stored),
            1 => Ok(Self::Lzs),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "invalid compress method",
            )),
        }
    }
}
