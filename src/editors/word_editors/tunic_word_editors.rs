use serde::{Deserialize, Serialize};
use std::cmp;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TunicWordEditor {
    word: TunicWord,
    glyph_editor: Option<GlyphEditor>,
    selected_glyph_index: Option<usize>,
    state: WordEditorState,
}

impl TunicWordEditor {
    pub fn new(word: TunicWord) -> Self {
        Self {
            word,
            glyph_editor: None,
            selected_glyph_index: None,
            state: WordEditorState::ModifySelectedGlyph,
        }
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

    pub fn with_word(self, word: TunicWord) -> Self {
        let new_index = match &self.selected_glyph_index {
            Some(current_index) => {
                let current_index = *current_index;
                let glyph_count: usize = word.glyphs().len();

                if glyph_count == 0 {
                    0
                } else if current_index > glyph_count {
                    glyph_count - 1
                } else {
                    current_index
                }
            }
            None => 0,
        };

        Self {
            word,
            ..self
        }
        .with_glyph_selected(new_index)
    }

    pub fn with_new_glyph_at_cursor(self) -> Self {
        let new_glyph: Glyph = DEFAULT_GLYPH;

        let new_index = self.selected_glyph_index.unwrap_or(0) + 1;

        let glyphs = self.word.glyphs();

        let left = glyphs.get(0..new_index).unwrap_or_default();
        let right = glyphs.get(new_index..).unwrap_or_default();

        let glyphs = [left, &[new_glyph], right].concat().to_vec();

        Self {
            word: self.word.clone().with_glyphs(glyphs).into(),
            ..self
        }
        .with_glyph_selected(new_index)
    }

    pub fn with_glyph_selected(self, index: usize) -> Self {
        let mut glyph_editor = self.glyph_editor.clone();
        let mut selected_glyph_index = self.selected_glyph_index;

        if let Some(glyph) = self.word.glyphs().get(index) {
            let glyph = *glyph;

            glyph_editor = Some(GlyphEditor { glyph });
            selected_glyph_index = Some(index);
        }

        Self {
            glyph_editor,
            selected_glyph_index,
            ..self
        }
    }

    pub fn with_glyph_selection_moved_forward(self, amount: usize) -> Self {
        let new_index = if let Some(index) = self.selected_glyph_index {
            cmp::min(self.word.glyphs().len(), index + amount)
        } else {
            0
        };

        self.with_glyph_selected(new_index)
    }

    pub fn with_glyph_selection_moved_backward(self, amount: usize) -> Self {
        let new_index = if let Some(index) = self.selected_glyph_index {
            if index >= amount {
                index - amount
            } else {
                0
            }
        } else {
            0
        };

        self.with_glyph_selected(new_index)
    }

    pub fn with_glyph_at_cursor_deleted(self) -> Self {
        let mut glyphs = self.word.glyphs();

        if let Some(selected_glyph_index) = self.selected_glyph_index {
            if glyphs.len() > 0 {
                glyphs.remove(selected_glyph_index);

                let new_index = if selected_glyph_index > 0 {
                    selected_glyph_index - 1
                } else {
                    0
                };

                Self {
                    word: self.word.with_glyphs(glyphs),
                    ..self
                }
                .with_glyph_selected(new_index)
            } else {
                self
            }
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

    pub fn word(&self) -> TunicWord {
        self.word.clone()
    }

    pub fn to_view(&self, params: WordViewParams) -> WordView {
        let glyph_views: Vec<GlyphView> = self.word.glyphs()
            .iter()
            .enumerate()
            .map(|(glyph_index, glyph)| {
                let selected_glyph =
                    if let Some(selected_glyph_index) = self.selected_glyph_index {
                        glyph_index == selected_glyph_index
                    } else {
                        false
                    };

                if params.selected && selected_glyph {
                    match self.glyph_editor.as_ref() {
                        Some(editor) => editor.to_view(true),
                        None => {
                            dbg!("Missing GlyphEditor");
                            dbg!(&self.glyph_editor);

                            GlyphEditor::new(glyph.clone()).to_view(false)
                        }
                    }
                } else {
                    GlyphEditor::new(*glyph).to_view(false)
                }
            })
            .collect();

        WordView {
            word: self.word.clone().into(),
            glyph_views,
            selected: params.selected,
            index: params.index,
            within_visible_range: params.within_visible_range,
            state: self.state,
        }
    }
}

impl AppliesEditorEvents for TunicWordEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match &event {
            EditorEvent::ToggleGlyphEditingMode => self.with_glyph_editing_mode_toggled(),
            EditorEvent::MoveGlyphCursorBackward => self.with_glyph_selection_moved_backward(1),
            EditorEvent::MoveGlyphCursorForward => self.with_glyph_selection_moved_forward(1),
            EditorEvent::AddNewGlyphToTunicWordAtCursor => self.with_new_glyph_at_cursor(),
            EditorEvent::DeleteGlyphAtCursor => self.with_glyph_at_cursor_deleted(),
            _ => {
                // TODO: Refactor to move all of this logic into GlyphEditor or the like
                if let Some(glyph_editor) = &self.glyph_editor {
                    let mut glyphs = self.word.glyphs();
                    let glyph_editor = glyph_editor.clone().apply(event);

                    if let Some(index) = self.selected_glyph_index {
                        if let Some(glyph) = glyphs.get_mut(index) {
                            *glyph = glyph_editor.glyph;
                        }
                    }

                    Self {
                        word: glyphs.into(),
                        glyph_editor: Some(glyph_editor),
                        ..self
                    }
                } else {
                    self
                }
            }
        }
    }
}
