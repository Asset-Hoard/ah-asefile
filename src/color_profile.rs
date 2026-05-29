use crate::{reader::AseReader, AsepriteParseError, Result};

/// Color profile information attached to an Aseprite file.
#[derive(Debug)]
pub struct ColorProfile {
    /// Which color profile kind this file uses.
    pub profile_type: ColorProfileType,
    /// Fixed gamma value, if the file specifies one. Encoded as a 32.16 fixed-point
    /// value in the file; exposed here already converted to `f64`.
    pub fixed_gamma: Option<f64>,
    /// Raw ICC profile bytes, present only when `profile_type` is
    /// [`ColorProfileType::ICC`].
    pub icc_profile: Option<Vec<u8>>,
}

/// Kind of color profile embedded in the file.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorProfileType {
    /// No color profile specified.
    None,
    /// Standard sRGB color space.
    Srgb,
    /// An embedded ICC profile; the raw bytes are in [`ColorProfile::icc_profile`].
    ICC,
}

pub(crate) fn parse_chunk(data: &[u8]) -> Result<ColorProfile> {
    let mut reader = AseReader::new(data);
    let profile_type = reader.word()?;
    let flags = reader.word()?;
    let raw_gamma = reader.dword()?;
    reader.skip_reserved(8)?;

    let profile_type = parse_color_profile_type(profile_type)?;
    let fixed_gamma = if flags & 1 != 0 {
        Some(raw_gamma as f64 / 65536.0)
    } else {
        None
    };

    let icc_profile = if profile_type == ColorProfileType::ICC {
        let icc_length = reader.dword()? as usize;
        Some(reader.take_bytes(icc_length)?)
    } else {
        None
    };

    Ok(ColorProfile {
        profile_type,
        fixed_gamma,
        icc_profile,
    })
}

fn parse_color_profile_type(id: u16) -> Result<ColorProfileType> {
    match id {
        0x0000 => Ok(ColorProfileType::None),
        0x0001 => Ok(ColorProfileType::Srgb),
        0x0002 => Ok(ColorProfileType::ICC),
        _ => Err(AsepriteParseError::UnsupportedFeature(format!(
            "Unknown color profile type: {}",
            id
        ))),
    }
}
