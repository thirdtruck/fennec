use serde::{Serialize, Deserialize};
use std::cmp;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EditorEvent {
    NoOp,
    ToggleSegmentOnActiveGlyph(Segment),
    MoveGlyphCursorRight,
    MoveGlyphCursorLeft,
    MoveWordCursorRight,
    MoveWordCursorLeft,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlyphEditor {
    glyph: Glyph,
}

impl GlyphEditor {
    pub fn with_segment_toggled(self, segment: usize) -> GlyphEditor {
        Self {
            glyph: self.glyph.with_toggled_segment(segment),
        }
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleSegmentOnActiveGlyph(segment) => self.with_segment_toggled(segment),
            _ => self
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
    active_glyph_index: Option<usize>,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word,
            glyph_editor: None,
            active_glyph_index: None,
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

#[derive(Default)]
pub struct SnippetEditorCallbacks {
    pub on_edit_word: Option<Box<dyn FnMut(Word) -> Vec<EditorEvent>>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SnippetEditor {
    active_snippet: Snippet,
    pub word_editor: Option<WordEditor>,
    active_word_index: Option<usize>,
}

impl SnippetEditor {
    pub fn new(snippet: Snippet) -> Self {
        Self {
            active_snippet: snippet,
            word_editor: None,
            active_word_index: None,
        }
    }

    pub fn on_input(&self, callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent>) -> EditorEvent {
        callback(self)
    }

    pub fn with_word_selected(self, index: usize) -> Self {
        let words = self.active_snippet.words.clone();

        if let Some(word) = words.get(index) {
            let editor = WordEditor::new(word.clone()).with_glyph_selected(0);

            SnippetEditor {
                word_editor: Some(editor),
                active_word_index: Some(index),
                ..self
            }
        } else {
            self
        }
    }

    pub fn with_word_selection_moved_forward(self, amount: usize) -> Self {
        if let Some(active_word_index) = self.active_word_index {
            let index = cmp::min(self.active_snippet.words.len(), active_word_index + amount);
            self.with_word_selected(index)
        } else {
            self
        }
    }

    pub fn with_word_selection_moved_backwards(self, amount: usize) -> Self {
        if let Some(active_word_index) = self.active_word_index {
            let index = if active_word_index >= amount {
                active_word_index - amount
            } else {
                0
            };

            self.with_word_selected(index)
        } else {
            self
        }
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::MoveWordCursorLeft => self.with_word_selection_moved_backwards(1),
            EditorEvent::MoveWordCursorRight => self.with_word_selection_moved_forward(1),
            _ => {
                if let Some(editor) = self.word_editor {
                    let word_editor = editor.apply(event);

                    let mut snippet = self.active_snippet.clone();

                    if let Some(index) = self.active_word_index {
                        if let Some(word) = snippet.words.get_mut(index) {
                            *word = word_editor.active_word.clone();
                        }
                    }

                    Self {
                        active_snippet: snippet,
                        word_editor: Some(word_editor),
                        ..self
                    }
                } else {
                    self
                }
            }
        }
    }

    pub fn render_with<R>(&self, mut renderer: R)
        where R: FnMut(SnippetView, usize)
    {
        let selected_glyph_index: Option<usize> = if let Some(editor) = &self.word_editor {
            editor.active_glyph_index
        } else {
            None
        };

        let word_views: Vec<WordView> = self.active_snippet.words
            .iter()
            .enumerate()
            .map(|(word_index, word)| {
                let word = word.clone();
                let selected_word = if let Some(active_index) = self.active_word_index {
                    word_index == active_index
                } else {
                    false
                };

                let glyph_views: Vec<GlyphView> = match word.clone() {
                    Word::Tunic(glyphs) => {
                        glyphs
                            .iter()
                            .enumerate()
                            .map(|(glyph_index, glyph)| {
                                let selected_glyph: bool = if let Some(index) = selected_glyph_index {
                                    index == glyph_index && selected_word
                                } else {
                                    false
                                };

                                GlyphView {
                                    glyph: *glyph,
                                    selected: selected_glyph,
                                }
                            })
                            .collect()

                    },
                    Word::English(_string) => todo!("Add support for English words"),
                };

                WordView {
                    word,
                    selected: selected_word,
                    glyph_views,
                }
            })
            .collect();

        let view = SnippetView {
            snippet: self.active_snippet.clone(),
            word_views,
            selected: true,
        };

        renderer(view, 0)
    }
}
