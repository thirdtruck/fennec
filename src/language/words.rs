use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Word {
    Tunic(Vec<Glyph>),
    English(String),
}

impl Word {
    pub fn is_empty(&self) -> bool {
        match self {
            Word::Tunic(glyphs) => glyphs.len() == 0,
            Word::English(string) => string.len() == 0,
        }
    }
}

impl Default for Word {
    fn default() -> Self {
        Self::Tunic(vec![])
    }
}

impl From<Vec<u16>> for Word {
    fn from(items: Vec<u16>) -> Self {
        let glyphs: Vec<Glyph> = items.iter().map(|c| Glyph(*c)).collect();

        Self::Tunic(glyphs)
    }
}

impl From<&[u16]> for Word {
    fn from(items: &[u16]) -> Self {
        let glyphs: Vec<Glyph> = items.iter().map(|c| Glyph(*c)).collect();

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

impl From<String> for Word {
    fn from(string: String) -> Self {
        Self::English(string)
    }
}
