use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub mod tunic_word_editors;
pub mod english_word_editors;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum SubEditorType {
    Tunic(TunicWordEditor),
    English(EnglishWordEditor),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WordEditor {
    sub_editor: SubEditorType,
}

pub struct WordEditorCallbacks {
    pub on_modify_selected_glyph: Box<dyn Fn(&GlyphEditor) -> EditorEvent>,
    pub on_modify_english_word: Box<dyn Fn(&EnglishWordEditor) -> EditorEvent>,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        let sub_editor = match &word.word_type {
            WordType::Tunic(word) => SubEditorType::Tunic(TunicWordEditor::new(word.clone()).with_glyph_selected(0)),
            WordType::English(word) => SubEditorType::English(EnglishWordEditor::new(word.clone())),
        };

        Self {
            sub_editor,
        }
    }

    pub fn selected_word(&self) -> Word {
        match &self.sub_editor {
            SubEditorType::English(editor) => editor.word().into(),
            SubEditorType::Tunic(editor) => editor.word().into(),
        }
    }

    pub fn on_input(&self, callbacks: WordEditorCallbacks) -> EditorEvent {
        match &self.sub_editor {
            SubEditorType::Tunic(editor) => editor.on_input(callbacks),
            SubEditorType::English(editor) => editor.on_input(callbacks),
        }
    }

    pub fn with_word(self, new_word: Word) -> Self {
        match &new_word.word_type {
            WordType::Tunic(tunic_word) => {
                let sub_editor = match &self.sub_editor {
                    SubEditorType::Tunic(editor) => SubEditorType::Tunic(editor.clone().with_word(tunic_word.clone())),
                    _ => SubEditorType::Tunic(TunicWordEditor::new(tunic_word.clone())),
                };


                Self {
                    sub_editor,
                    ..self
                }
            }
            WordType::English(english_word) => Self {
                sub_editor: SubEditorType::English(english_word.into()),
                ..self
            },
        }
    }

    pub fn to_view(&self, params: WordViewParams) -> WordView {
        match &self.sub_editor {
            SubEditorType::Tunic(editor) => editor.to_view(params),
            SubEditorType::English(editor) => editor.to_view(params),
        }
    }
}

impl AppliesEditorEvents for WordEditor {
    fn apply(self, event: EditorEvent) -> Self {
        let sub_editor = match &self.sub_editor {
            SubEditorType::English(editor) => SubEditorType::English(editor.clone().apply(event.clone())),
            SubEditorType::Tunic(editor) => SubEditorType::Tunic(editor.clone().apply(event.clone())),
        };

        Self {
            sub_editor,
            ..self
        }

    }
}
