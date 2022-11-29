use std::convert::From;

pub const ALL_SEGMENTS: u16 = 0b1111_1111_1111_1110;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    ManualPageNumber(usize),
    ScreenshotFilename(String),
    Other(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
            00 => 0b1000_0000_0000_0000,
            01 => 0b0100_0000_0000_0000,
            02 => 0b0010_0000_0000_0000,
            03 => 0b0001_0000_0000_0000,
            04 => 0b0000_1000_0000_0000,
            05 => 0b0000_0100_0000_0000,
            06 => 0b0000_0010_0000_0000,
            07 => 0b0000_0001_0000_0000,
            08 => 0b0000_0000_1000_0000,
            09 => 0b0000_0000_0100_0000,
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
            0b1000_0000_0000_0000 => 00,
            0b0100_0000_0000_0000 => 01,
            0b0010_0000_0000_0000 => 02,
            0b0001_0000_0000_0000 => 03,
            0b0000_1000_0000_0000 => 04,
            0b0000_0100_0000_0000 => 05,
            0b0000_0010_0000_0000 => 06,
            0b0000_0001_0000_0000 => 07,
            0b0000_0000_1000_0000 => 08,
            0b0000_0000_0100_0000 => 09,
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
            00 => 0b1000_0000_0000_0000,
            01 => 0b0100_0000_0000_0000,
            02 => 0b0010_0000_0000_0000,
            03 => 0b0001_0000_0000_0000,
            04 => 0b0000_1000_0000_0000,
            05 => 0b0000_0100_0000_0000,
            06 => 0b0000_0010_0000_0000,
            07 => 0b0000_0001_0000_0000,
            08 => 0b0000_0000_1000_0000,
            09 => 0b0000_0000_0100_0000,
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
        println!("Glyph to toggle: {:?}", self);
        println!("Glyph code to toggle: {:#b}", self.0);

        let mask = Self::mask_from_usize(index);

        println!("Mask: {:#b}", mask);

        let new_code = if self.includes_segment(Self::segment_index_from_mask(mask)) {
            self.0 ^ mask
        } else {
            self.0 | mask
        };

        println!("New code: {:#b}", mask);

        Self(new_code)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Word {
    Tunic(Vec<Glyph>),
    English(String),
}

impl From<Vec<u16>> for Word {
    fn from(items: Vec<u16>) -> Self {
        let glyphs: Vec<Glyph> = items
            .iter()
            .map(|g| (*g).into())
            .collect();

        Self::Tunic(glyphs)
    }
}

impl From<&[u16]> for Word {
    fn from(items: &[u16]) -> Self {
        let glyphs: Vec<Glyph> = items
            .iter()
            .map(|g| (*g).into())
            .collect();

        Self::Tunic(glyphs)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snippet {
    pub words: Vec<Word>,
    pub source: Option<Source>,
}
