use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Glyph(pub u16);

impl From<u16> for Glyph {
    fn from(item: u16) -> Self {
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
    pub fn includes_segment(&self, segment: u16) -> bool {
        let mask: u16 = match segment {
            0 => 0b1000_0000_0000_0000,
            1 => 0b0100_0000_0000_0000,
            2 => 0b0010_0000_0000_0000,
            3 => 0b0001_0000_0000_0000,
            4 => 0b0000_1000_0000_0000,
            5 => 0b0000_0100_0000_0000,
            6 => 0b0000_0010_0000_0000,
            7 => 0b0000_0001_0000_0000,
            8 => 0b0000_0000_1000_0000,
            9 => 0b0000_0000_0100_0000,
            10 => 0b0000_0000_0010_0000,
            11 => 0b0000_0000_0001_0000,
            12 => 0b0000_0000_0000_1000,
            13 => 0b0000_0000_0000_0100,
            14 => 0b0000_0000_0000_0010,
            15 => 0b0000_0000_0000_0001,
            _ => panic!("Unexpected segment index: {}", segment),
        };

        (mask & self.0) > 0
    }

    pub fn segment_index_from_mask(mask: u16) -> u16 {
        match mask {
            0b1000_0000_0000_0000 => 0,
            0b0100_0000_0000_0000 => 1,
            0b0010_0000_0000_0000 => 2,
            0b0001_0000_0000_0000 => 3,
            0b0000_1000_0000_0000 => 4,
            0b0000_0100_0000_0000 => 5,
            0b0000_0010_0000_0000 => 6,
            0b0000_0001_0000_0000 => 7,
            0b0000_0000_1000_0000 => 8,
            0b0000_0000_0100_0000 => 9,
            0b0000_0000_0010_0000 => 10,
            0b0000_0000_0001_0000 => 11,
            0b0000_0000_0000_1000 => 12,
            0b0000_0000_0000_0100 => 13,
            0b0000_0000_0000_0010 => 14,
            0b0000_0000_0000_0001 => 15,
            _ => panic!("Unexpected glyph mask value: {}", mask),
        }
    }

    pub fn mask_from_usize(index: usize) -> u16 {
        match index {
            0 => 0b1000_0000_0000_0000,
            1 => 0b0100_0000_0000_0000,
            2 => 0b0010_0000_0000_0000,
            3 => 0b0001_0000_0000_0000,
            4 => 0b0000_1000_0000_0000,
            5 => 0b0000_0100_0000_0000,
            6 => 0b0000_0010_0000_0000,
            7 => 0b0000_0001_0000_0000,
            8 => 0b0000_0000_1000_0000,
            9 => 0b0000_0000_0100_0000,
            10 => 0b0000_0000_0010_0000,
            11 => 0b0000_0000_0001_0000,
            12 => 0b0000_0000_0000_1000,
            13 => 0b0000_0000_0000_0100,
            14 => 0b0000_0000_0000_0010,
            15 => 0b0000_0000_0000_0001,
            _ => panic!("Unexpected segment index: {}", index),
        }
    }

    pub fn with_toggled_segment(&self, index: usize) -> Self {
        let mask = Self::mask_from_usize(index);

        let new_code = if self.includes_segment(Self::segment_index_from_mask(mask)) {
            self.0 ^ mask
        } else {
            self.0 | mask
        };

        Self(new_code)
    }
}
