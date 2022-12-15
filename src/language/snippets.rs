use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    pub words: Vec<Word>,
    pub source: Option<Source>,
}

impl Snippet {
    pub fn starting_snippet() -> Self {
        // Arbitrary starting glyph value
        let glyph: Glyph = (0x10).into();
        let words = vec![Word::Tunic(vec![glyph])];
        let source = Some(Source::Other("ADD_SOURCE_HERE".into()));

        Self { words, source }
    }
}

impl From<Vec<Word>> for Snippet {
    fn from(words: Vec<Word>) -> Self {
        Self {
            words,
            source: None,
        }
    }
}
