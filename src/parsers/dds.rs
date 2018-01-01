use core::*;
use errors::*;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use webgl::{TextureCompression,PixelFormat};

pub fn parse(dds: &[u8]) -> Result<TextureData> {
    let mut buf = Cursor::new(dds);
    // ;
    // 542327876
    let magic = buf.read_u32::<LittleEndian>()
        .map_err(|_| "Error::IOError")?;
    if magic != 542327876 {
        return Err(ErrorKind::InvalidDDSFile.into());
    };
    buf.set_position(12);
    let width = buf.read_u32::<LittleEndian>()
        .chain_err(|| "little endian read failed")?;
    let height = buf.read_u32::<LittleEndian>()
        .chain_err(|| "little endian read failed")?;

    Ok(TextureData::new(
        width as _,
        height as _,
        dds.to_vec(),
        Some(TextureCompression::RgbaDxt5),
        PixelFormat::Rgba,
    ))
}
