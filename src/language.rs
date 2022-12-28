use serde::{Deserialize, Serialize};
use std::fmt;

pub mod glyphs;
pub mod notebooks;
pub mod snippets;
pub mod words;

pub type Segment = usize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Source {
    ManualPageNumber(usize),
    ScreenshotFilename(String),
    Other(String),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::ManualPageNumber(page_number) => write!(f, "ManualPageNumber({})", page_number),
            Source::ScreenshotFilename(filename) => write!(f, "ScreenshotFilename({})", filename),
            Source::Other(text) => write!(f, "Other({})", text),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note(String);

impl From<&str> for Note {
    fn from(text: &str) -> Self {
        Note(text.to_string())
    }
}
