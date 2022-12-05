use serde::{Serialize, Deserialize};
use std::convert::From;

pub type Segment = usize;

#[allow(dead_code)]
pub const ALL_SEGMENTS: u16 = 0b1111_1111_1111_1110;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Source {
    ManualPageNumber(usize),
    ScreenshotFilename(String),
    Other(String),
}

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Word {
    Tunic(Vec<Glyph>),
    English(String),
}

impl Default for Word {
    fn default() -> Self { Self::Tunic(vec![]) }
}

impl From<Vec<u16>> for Word {
    fn from(items: Vec<u16>) -> Self {
        let glyphs: Vec<Glyph> = items
            .iter()
            .map(|c| Glyph(*c))
            .collect();

        Self::Tunic(glyphs)
    }
}

impl From<&[u16]> for Word {
    fn from(items: &[u16]) -> Self {
        let glyphs: Vec<Glyph> = items
            .iter()
            .map(|c| Glyph(*c))
            .collect();

        Self::Tunic(glyphs)
    }
}

impl From<Vec<Glyph>> for Word {
    fn from(glyphs: Vec<Glyph>) -> Self {
        Self::Tunic(glyphs)
    }
}

impl From<Glyph> for Word {
    fn from(glyph: Glyph) -> Self {
        Self::Tunic(vec![glyph])
    }
}

impl From<&str> for Word {
    fn from(string: &str) -> Self {
        Self::English(string.to_string())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    pub words: Vec<Word>,
    pub source: Option<Source>,
}

impl From<Vec<Word>> for Snippet {
    fn from(words: Vec<Word>) -> Self {
        Self {
            words,
            source: None,
        }
    }
}
