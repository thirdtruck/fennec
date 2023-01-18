use serde::{Deserialize, Serialize};
use std::cmp;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TunicWordEditor {
    word: TunicWord,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct EnglishWordEditor {
    word: EnglishWord,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum SubEditorType {
    Tunic(TunicWordEditor),
    English(EnglishWordEditor),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WordEditorState {
    ModifySelectedGlyph,
    ModifyGlyphSet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WordEditor {
    selected_word: Word,
    glyph_editor: Option<GlyphEditor>,
    selected_glyph_index: Option<usize>,
    state: WordEditorState,
    sub_editor: SubEditorType,
}

pub struct WordEditorCallbacks {
    pub on_modify_selected_glyph: Box<dyn Fn(&GlyphEditor) -> EditorEvent>,
    pub on_modify_glyph_set: Box<dyn Fn(&WordEditor) -> EditorEvent>,
}

// TODO: Extract TunicWordEditor and EnglishWordEditor
impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            selected_word: word,
            glyph_editor: None,
            selected_glyph_index: None,
            state: WordEditorState::ModifySelectedGlyph,
            sub_editor: SubEditorType::None,
        }
        .with_glyph_selected(0)
    }

    pub fn selected_word(&self) -> Word {
        self.selected_word.clone()
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

    pub fn with_word(self, word: Word) -> Self {
        match &self.selected_word.word_type {
            WordType::Tunic(tunic_word) => {
                let new_index = match self.selected_glyph_index {
                    Some(current_index) => {
                        let glyph_count = tunic_word.glyphs().len();

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
                    selected_word: word,
                    ..self
                }
                .with_glyph_selected(new_index)
            }
            WordType::English(_) => Self {
                selected_word: word,
                glyph_editor: None,
                selected_glyph_index: None,
                ..self
            },
        }
    }

    pub fn with_new_glyph_at_cursor(self) -> Self {
        match &self.selected_word.word_type {
            WordType::Tunic(word) => {
                let new_glyph: Glyph = DEFAULT_GLYPH;

                let new_index = self.selected_glyph_index.unwrap_or(0) + 1;

                let glyphs = word.glyphs();

                let left = glyphs.get(0..new_index).unwrap_or_default();
                let right = glyphs.get(new_index..).unwrap_or_default();

                let glyphs = [left, &[new_glyph], right].concat().to_vec();

                let selected_word: Word = word.clone().with_glyphs(glyphs).into();

                Self {
                    selected_word,
                    ..self
                }
                .with_glyph_selected(new_index)
            }
            WordType::English(_) => self,
        }
    }

    pub fn with_glyph_selected(self, index: usize) -> Self {
        let mut glyph_editor = self.glyph_editor.clone();
        let mut selected_glyph_index = self.selected_glyph_index;

        if let WordType::Tunic(word) = &self.selected_word.word_type {
            if let Some(glyph) = word.glyphs().get(index) {
                let glyph = *glyph;

                glyph_editor = Some(GlyphEditor { glyph });
                selected_glyph_index = Some(index);
            }
        }

        Self {
            glyph_editor,
            selected_glyph_index,
            ..self
        }
    }

    pub fn with_glyph_selection_moved_forward(self, amount: usize) -> Self {
        if let WordType::Tunic(word) = &self.selected_word.word_type {
            let new_index = if let Some(index) = self.selected_glyph_index {
                cmp::min(word.glyphs().len(), index + amount)
            } else {
                0
            };

            self.with_glyph_selected(new_index)
        } else {
            self
        }
    }

    pub fn with_glyph_selection_moved_backward(self, amount: usize) -> Self {
        if let WordType::Tunic(_) = &self.selected_word.word_type {
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
        } else {
            self
        }
    }

    pub fn with_glyph_at_cursor_deleted(self) -> Self {
        if let WordType::Tunic(word) = &self.selected_word.word_type {
            let mut glyphs = word.glyphs();

            if let Some(selected_glyph_index) = self.selected_glyph_index {
                if glyphs.len() > 0 {
                    glyphs.remove(selected_glyph_index);

                    let new_index = if selected_glyph_index > 0 {
                        selected_glyph_index - 1
                    } else {
                        0
                    };

                    let selected_word = word.clone();
                    let selected_word: Word = selected_word.with_glyphs(glyphs).into();

                    Self {
                        selected_word,
                        ..self
                    }
                    .with_glyph_selected(new_index)
                } else {
                    self
                }
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

    pub fn to_view(&self, params: WordViewParams) -> WordView {
        match &self.selected_word.word_type {
            WordType::Tunic(word) => {
                let glyph_views: Vec<GlyphView> = word.glyphs()
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
                    word: self.selected_word.clone(),
                    glyph_views,
                    selected: params.selected,
                    index: params.index,
                    within_visible_range: params.within_visible_range,
                    state: self.state,
                }
            }
            WordType::English(_) => WordView {
                word: self.selected_word.clone(),
                glyph_views: vec![],
                selected: params.selected,
                index: params.index,
                within_visible_range: params.within_visible_range,
                state: self.state,
            },
        }
    }
}

impl AppliesEditorEvents for WordEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleGlyphEditingMode => self.with_glyph_editing_mode_toggled(),
            EditorEvent::MoveGlyphCursorBackward => self.with_glyph_selection_moved_backward(1),
            EditorEvent::MoveGlyphCursorForward => self.with_glyph_selection_moved_forward(1),
            EditorEvent::AddNewGlyphToTunicWordAtCursor => self.with_new_glyph_at_cursor(),
            EditorEvent::DeleteGlyphAtCursor => self.with_glyph_at_cursor_deleted(),
            _ => {
                // TODO: Refactor to move all of this logic into GlyphEditor or the like
                if let Some(editor) = self.glyph_editor {
                    let glyph_editor = editor.apply(event);

                    let new_word = match &self.selected_word.word_type {
                        WordType::Tunic(word) => {
                            let mut glyphs = word.glyphs();

                            if let Some(index) = self.selected_glyph_index {
                                if let Some(glyph) = glyphs.get_mut(index) {
                                    *glyph = glyph_editor.glyph;
                                }
                            }

                            word.clone().with_glyphs(glyphs).into()
                        }
                        _ => self.selected_word.clone(),
                    };

                    Self {
                        selected_word: new_word,
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
