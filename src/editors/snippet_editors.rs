use serde::{Serialize, Deserialize};

use std::cmp;

use crate::prelude::*;

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
