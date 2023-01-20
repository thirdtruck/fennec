use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnglishWordEditor {
    word: EnglishWord,
}

impl EnglishWordEditor {
    pub fn to_view(&self, params: WordViewParams) -> WordView {
        WordView {
            word: self.word.clone().into(),
            glyph_views: vec![],
            selected: params.selected,
            index: params.index,
            within_visible_range: params.within_visible_range,
        }
    }
}

impl EnglishWordEditor {
    pub fn new(word: EnglishWord) -> Self {
        Self { word }
    }

    pub fn on_input(&self, _callbacks: WordEditorCallbacks) -> EditorEvent {
        EditorEvent::NoOp
    }

    pub fn word(&self) -> EnglishWord {
        self.word.clone()
    }
}

impl AppliesEditorEvents for EnglishWordEditor {
    fn apply(self, _event: EditorEvent) -> Self {
        self
    }
}

impl From<&EnglishWord> for EnglishWordEditor {
    fn from(word: &EnglishWord) -> Self {
        EnglishWordEditor {
            word: word.clone(),
        }
    }
}

impl From<EnglishWord> for EnglishWordEditor {
    fn from(word: EnglishWord) -> Self {
        EnglishWordEditor {
            word,
        }
    }
}
