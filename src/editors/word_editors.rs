use serde::{Deserialize, Serialize};
use std::cmp;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordEditorState {
    ModifySelectedGlyph,
    ModifyGlyphSet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
    active_glyph_index: Option<usize>,
    state: WordEditorState,
}

pub struct WordEditorCallbacks {
    pub on_modify_selected_glyph: Box<dyn Fn(&GlyphEditor) -> EditorEvent>,
    pub on_modify_glyph_set: Box<dyn Fn(&WordEditor) -> EditorEvent>,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word,
            glyph_editor: None,
            active_glyph_index: None,
            state: WordEditorState::ModifySelectedGlyph,
        }
    }

    pub fn selected_word(&self) -> Word {
        self.active_word.clone()
    }

    pub fn on_input(&self, callbacks: WordEditorCallbacks) -> EditorEvent {
        match self.state {
            WordEditorState::ModifySelectedGlyph => {
                if let Some(editor) = &self.glyph_editor {
                    (callbacks.on_modify_selected_glyph)(editor)
                } else {
                    EditorEvent::NoOp
                }
            }
            WordEditorState::ModifyGlyphSet => {
                if let Some(_editor) = &self.glyph_editor {
                    (callbacks.on_modify_glyph_set)(self)
                } else {
                    EditorEvent::NoOp
                }
            }
        }
    }

    pub fn with_glyph_selected(self, index: usize) -> Self {
        let mut glyph_editor = self.glyph_editor.clone();
        let mut active_glyph_index = self.active_glyph_index;

        if let Word::Tunic(glyphs) = &self.active_word {
            if let Some(glyph) = glyphs.get(index) {
                let glyph = *glyph;

                glyph_editor = Some(GlyphEditor { glyph });
                active_glyph_index = Some(index);
            }
        }

        Self {
            glyph_editor,
            active_glyph_index,
            ..self
        }
    }

    pub fn with_glyph_selection_moved_forward(self, amount: usize) -> Self {
        if let Word::Tunic(glyphs) = &self.active_word {
            let new_index = if let Some(index) = self.active_glyph_index {
                cmp::min(glyphs.len(), index + amount)
            } else {
                0
            };

            self.with_glyph_selected(new_index)
        } else {
            self
        }
    }

    pub fn with_glyph_selection_moved_backward(self, amount: usize) -> Self {
        if let Word::Tunic(_glyphs) = &self.active_word {
            let new_index = if let Some(index) = self.active_glyph_index {
                if index >= amount {
                    index - amount
                } else {
                    0
                }
            } else {
                0
            };

            self.with_glyph_selected(new_index)
        } else {
            self
        }
    }

    pub fn with_glyph_editing_mode_toggled(self) -> Self {
        let state = match &self.state {
            WordEditorState::ModifyGlyphSet => WordEditorState::ModifySelectedGlyph,
            WordEditorState::ModifySelectedGlyph => WordEditorState::ModifyGlyphSet,
        };

        Self { state, ..self }
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleGlyphEditingMode => self.with_glyph_editing_mode_toggled(),
            EditorEvent::MoveGlyphCursorBackward => self.with_glyph_selection_moved_backward(1),
            EditorEvent::MoveGlyphCursorForward => self.with_glyph_selection_moved_forward(1),
            _ => {
                if let Some(editor) = self.glyph_editor {
                    let glyph_editor = editor.apply(event);

                    let new_word = match self.active_word.clone() {
                        Word::Tunic(mut glyphs) => {
                            if let Some(index) = self.active_glyph_index {
                                if let Some(glyph) = glyphs.get_mut(index) {
                                    *glyph = glyph_editor.glyph;
                                }
                            }

                            Word::Tunic(glyphs)
                        }
                        _ => todo!("Add support for other word types"),
                    };

                    Self {
                        active_word: new_word,
                        glyph_editor: Some(glyph_editor),
                        ..self
                    }
                } else {
                    self
                }
            }
        }
    }

    pub fn to_view(&self, selected: bool) -> WordView {
        match &self.active_word {
            Word::Tunic(glyphs) => {
                let glyph_views: Vec<GlyphView> = glyphs
                    .iter()
                    .enumerate()
                    .map(|(glyph_index, glyph)| {
                        let selected = if let Some(active_glyph_index) = self.active_glyph_index {
                            glyph_index == active_glyph_index
                        } else {
                            false
                        };

                        if selected {
                            self.glyph_editor
                                .as_ref()
                                .expect("Missing GlyphEditor")
                                .to_view(true)
                        } else {
                            GlyphEditor::new(*glyph).to_view(false)
                        }
                    })
                    .collect();

                WordView {
                    word: self.active_word.clone(),
                    glyph_views,
                    selected,
                    state: self.state,
                }
            }
            Word::English(_) => todo!("Add support for English words"),
        }
    }
}
