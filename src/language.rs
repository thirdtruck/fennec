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
