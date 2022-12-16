use serde::{Deserialize, Serialize};

use std::convert::From;
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GlyphError {
    glyph: Glyph,
    description: String,
}

impl GlyphError {
    pub fn new(glyph: Glyph, description: String) -> Self {
        Self { glyph, description }
    }
}

impl fmt::Display for GlyphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GlyphError({}): {}", self.glyph, self.description)
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
    pub fn includes_segment(&self, segment: u16) -> Option<bool> {
        let mask: Option<u16> = match segment {
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
            Some((mask & self.0) > 0)
        } else {
            None
        }
    }

    pub fn segment_index_from_mask(mask: u16) -> Option<u16> {
        match mask {
            0b1000_0000_0000_0000 => Some(0),
            0b0100_0000_0000_0000 => Some(1),
            0b0010_0000_0000_0000 => Some(2),
            0b0001_0000_0000_0000 => Some(3),
            0b0000_1000_0000_0000 => Some(4),
            0b0000_0100_0000_0000 => Some(5),
            0b0000_0010_0000_0000 => Some(6),
            0b0000_0001_0000_0000 => Some(7),
            0b0000_0000_1000_0000 => Some(8),
            0b0000_0000_0100_0000 => Some(9),
            0b0000_0000_0010_0000 => Some(10),
            0b0000_0000_0001_0000 => Some(11),
            0b0000_0000_0000_1000 => Some(12),
            0b0000_0000_0000_0100 => Some(13),
            0b0000_0000_0000_0010 => Some(14),
            0b0000_0000_0000_0001 => Some(15),
            _ => None,
        }
    }

    pub fn mask_from_usize(index: usize) -> Option<u16> {
        match index {
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
        }
    }

    pub fn with_toggled_segment(&self, index: usize) -> Result<Self, GlyphError> {
        // TODO: Use #or_else or like here for conciseness

        if let Some(mask) = Self::mask_from_usize(index) {
            if let Some(segment_index) = Self::segment_index_from_mask(mask) {
                if let Some(new_code) = self.includes_segment(segment_index) {
                    let new_code = if new_code {
                        self.0 ^ mask
                    } else {
                        self.0 | mask
                    };

                    Ok(Self(new_code))
                } else {
                    Err(GlyphError::new(*self, format!("Unexpected segment index: {}", segment_index)))
                }
            } else {
                Err(GlyphError::new(*self, format!("Invalid segment mask: {}", mask)))
            }
        } else {
            Err(GlyphError::new(*self, format!("Unexpected segment index: {}", index)))
        }
    }
}
