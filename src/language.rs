use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note(String);

impl From<&str> for Note {
    fn from(text: &str) -> Self {
        Note(text.to_string())
    }
}
