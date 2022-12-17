use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    pub words: Vec<Word>,
    pub source: Option<Source>,
    pub description: String,
}

impl Snippet {
    pub fn starting_snippet() -> Self {
        // Arbitrary starting glyph value
        let code: u16 = 0x10;
        let glyph: Glyph = code.into();
        let words = vec![Word::Tunic(vec![glyph])];
        let source = Some(Source::Other("ADD_SOURCE_HERE".into()));
        let description = "ADD_DESCRIPTION_HERE".into();

        Self {
            words,
            source,
            description,
        }
    }
}
