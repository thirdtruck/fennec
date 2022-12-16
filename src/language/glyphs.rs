use serde::{Deserialize, Serialize};

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GlyphError {
    description: String,
}

impl GlyphError {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

impl fmt::Display for GlyphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GlyphError: {}", self.description)
    }
}

impl Error for GlyphError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Glyph(pub u16);

impl fmt::Display for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Glyph({})", self.0)
    }
}

impl From<u16> for Glyph {
    fn from(item: u16) -> Self {
        Self(item)
    }
}

impl From<i32> for Glyph {
    fn from(item: i32) -> Self {
        let item: u16 = item.try_into().unwrap();
        Self(item)
    }
}

impl From<usize> for Glyph {
    fn from(item: usize) -> Self {
        let item: u16 = item.try_into().unwrap();
        Self(item)
    }
}

impl Glyph {
    pub fn includes_segment(&self, index: u16) -> Result<bool, GlyphError> {
        let mask: Option<u16> = match index {
            0 => Some(0b1000_0000_0000_0000),
            1 => Some(0b0100_0000_0000_0000),
            2 => Some(0b0010_0000_0000_0000),
            3 => Some(0b0001_0000_0000_0000),
            4 => Some(0b0000_1000_0000_0000),
            5 => Some(0b0000_0100_0000_0000),
            6 => Some(0b0000_0010_0000_0000),
            7 => Some(0b0000_0001_0000_0000),
            8 => Some(0b0000_0000_1000_0000),
            9 => Some(0b0000_0000_0100_0000),
            10 => Some(0b0000_0000_0010_0000),
            11 => Some(0b0000_0000_0001_0000),
            12 => Some(0b0000_0000_0000_1000),
            13 => Some(0b0000_0000_0000_0100),
            14 => Some(0b0000_0000_0000_0010),
            15 => Some(0b0000_0000_0000_0001),
            _ => None,
        };

        if let Some(mask) = mask {
            Ok((mask & self.0) > 0)
        } else {
            Err(GlyphError::new(format!("Unexpected segment index: {}", index)))
        }
    }

    pub fn segment_index_from_mask(mask: u16) -> Result<u16, GlyphError> {
        match mask {
            0b1000_0000_0000_0000 => Ok(0),
            0b0100_0000_0000_0000 => Ok(1),
            0b0010_0000_0000_0000 => Ok(2),
            0b0001_0000_0000_0000 => Ok(3),
            0b0000_1000_0000_0000 => Ok(4),
            0b0000_0100_0000_0000 => Ok(5),
            0b0000_0010_0000_0000 => Ok(6),
            0b0000_0001_0000_0000 => Ok(7),
            0b0000_0000_1000_0000 => Ok(8),
            0b0000_0000_0100_0000 => Ok(9),
            0b0000_0000_0010_0000 => Ok(10),
            0b0000_0000_0001_0000 => Ok(11),
            0b0000_0000_0000_1000 => Ok(12),
            0b0000_0000_0000_0100 => Ok(13),
            0b0000_0000_0000_0010 => Ok(14),
            0b0000_0000_0000_0001 => Ok(15),
            _ => Err(GlyphError::new(format!("Invalid segment mask: {}", mask))),
        }
    }

    pub fn mask_from_usize(index: usize) -> Result<u16, GlyphError> {
        match index {
            0 => Ok(0b1000_0000_0000_0000),
            1 => Ok(0b0100_0000_0000_0000),
            2 => Ok(0b0010_0000_0000_0000),
            3 => Ok(0b0001_0000_0000_0000),
            4 => Ok(0b0000_1000_0000_0000),
            5 => Ok(0b0000_0100_0000_0000),
            6 => Ok(0b0000_0010_0000_0000),
            7 => Ok(0b0000_0001_0000_0000),
            8 => Ok(0b0000_0000_1000_0000),
            9 => Ok(0b0000_0000_0100_0000),
            10 => Ok(0b0000_0000_0010_0000),
            11 => Ok(0b0000_0000_0001_0000),
            12 => Ok(0b0000_0000_0000_1000),
            13 => Ok(0b0000_0000_0000_0100),
            14 => Ok(0b0000_0000_0000_0010),
            15 => Ok(0b0000_0000_0000_0001),
            _ => Err(GlyphError::new(format!("Unexpected segment index: {}", index))),
        }
    }

    fn with_mask_applied(&self, mask: u16, included: bool) -> Self {
        let new_code = if included {
            self.0 ^ mask
        } else {
            self.0 | mask
        };

        Self(new_code)
    }

    pub fn with_toggled_segment(&self, index: usize) -> Result<Self, GlyphError> {
        let mask = Self::mask_from_usize(index);

        let included = mask.clone()
            .and_then(|m| Self::segment_index_from_mask(m))
            .and_then(|si| self.includes_segment(si))
            .map_or(false, |incld| incld);

        mask.map(|m| self.with_mask_applied(m, included))
    }
}
