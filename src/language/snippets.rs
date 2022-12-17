use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    pub words: Vec<Word>,
    pub source: Option<Source>,
    pub description: String,
    pub notes: Vec<Note>,
    pub transcribed: bool,
}

impl Snippet {
    pub fn starting_snippet() -> Self {
        let words = vec![Word::Tunic(vec![DEFAULT_GLYPH])];
        let source = Some(Source::Other("ADD_SOURCE_HERE".into()));
        let description = "ADD_DESCRIPTION_HERE".into();
        let note: Note = "ADD_NOTE_HERE".into();

        Self {
            words,
            source,
            description,
            notes: vec![note],
            transcribed: false,
        }
    }
}
