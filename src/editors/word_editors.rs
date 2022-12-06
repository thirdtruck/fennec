use serde::{Serialize, Deserialize};
use std::cmp;

use crate::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WordEditor {
    active_word: Word,
    pub glyph_editor: Option<GlyphEditor>,
    pub active_glyph_index: Option<usize>,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word,
            glyph_editor: None,
            active_glyph_index: None,
        }
    }

    pub fn selected_word(&self) -> Word {
        self.active_word.clone()
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

    pub fn with_glyph_selection_moved_backwards(self, amount: usize) -> Self {
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

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::MoveGlyphCursorLeft => {
                self.with_glyph_selection_moved_backwards(1)
            },
            EditorEvent::MoveGlyphCursorRight => {
                self.with_glyph_selection_moved_forward(1)
            },
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
                        },
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
            },
        }
    }
}
