use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::{
    constants::{APPIMAGE_MAGIC_BYTES, ELF_MAGIC_BYTES, FLATIMAGE_MAGIC_BYTES, WRAPPE_MAGIC_BYTES},
    SoarResult,
};

pub mod appimage;
pub mod common;
pub mod wrappe;

#[derive(Debug, PartialEq, Eq)]
pub enum PackageFormat {
    AppImage,
    FlatImage,
    Wrappe,
    ELF,
    Unknown,
}

pub fn get_file_type<T>(file: &mut BufReader<T>) -> SoarResult<PackageFormat>
where
    T: Read + Seek,
{
    let mut magic_bytes = [0u8; 12];
    file.read_exact(&mut magic_bytes)?;

    if magic_bytes[8..] == APPIMAGE_MAGIC_BYTES {
        return Ok(PackageFormat::AppImage);
    }
    if magic_bytes[8..] == FLATIMAGE_MAGIC_BYTES {
        return Ok(PackageFormat::FlatImage);
    }

    let start = file.seek(SeekFrom::End(0))?.wrapping_sub(801);
    file.rewind()?;

    if file.seek(SeekFrom::Start(start)).is_ok() {
        let mut magic_bytes = [0u8; 8];
        file.read_exact(&mut magic_bytes)?;
        file.rewind()?;
        if magic_bytes[0..8] == WRAPPE_MAGIC_BYTES {
            return Ok(PackageFormat::Wrappe);
        }
    }

    if magic_bytes[..4] == ELF_MAGIC_BYTES {
        return Ok(PackageFormat::ELF);
    }

    return Ok(PackageFormat::Unknown);
}
