//use bracket_lib::prelude::*;

use std::convert::From;

#[derive(Clone, Debug, PartialEq)]
enum Source {
    ManualPageNumber(usize),
    ScreenshotFilename(String),
    Other(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Glyph(u16);

impl From<u16> for Glyph {
    fn from(item: u16) -> Self {
        Self(item)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Word {
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
struct Snippet {
    words: Vec<Word>,
    source: Option<Source>,
}

fn main() {
    let glyph: Glyph = (0xAF).into();
    let word1 = Word::Tunic(vec![glyph]);
    let word2 = Word::English("Testing".into());
    let word3: Word = vec![0x01, 0x11, 0xF1].into();
    let snippet = Snippet {
        words: vec![word1, word2, word3],
        source: Some(Source::Other("Example snippet".into())),
    };

    println!("Hello, world!");
    println!("Here's your glyph! {:?}", snippet);
}
