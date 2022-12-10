use serde::{Deserialize, Serialize};

use std::cmp;

use crate::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SnippetEditor {
    active_snippet: Snippet,
    word_editor: Option<WordEditor>,
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

    pub fn on_word_editor_input(&self, callbacks: WordEditorCallbacks) -> EditorEvent {
        if let Some(editor) = &self.word_editor {
            editor.on_input(callbacks)
        } else {
            EditorEvent::NoOp
        }
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

    pub fn with_word_selection_moved_backward(self, amount: usize) -> Self {
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
            EditorEvent::MoveWordCursorBackward => self.with_word_selection_moved_backward(1),
            EditorEvent::MoveWordCursorForward => self.with_word_selection_moved_forward(1),
            _ => {
                if let Some(editor) = self.word_editor {
                    let word_editor = editor.apply(event);

                    let mut snippet = self.active_snippet.clone();

                    if let Some(index) = self.active_word_index {
                        if let Some(word) = snippet.words.get_mut(index) {
                            *word = word_editor.selected_word();
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
    where
        R: FnMut(SnippetView, usize),
    {
        let word_views: Vec<WordView> = self
            .active_snippet
            .words
            .iter()
            .enumerate()
            .map(|(word_index, word)| {
                let selected = if let Some(active_word_index) = self.active_word_index {
                    word_index == active_word_index
                } else {
                    false
                };

                if selected {
                    self.word_editor
                        .as_ref()
                        .expect("Missing WordEditor")
                        .to_view(true)
                } else {
                    WordEditor::new(word.clone()).to_view(false)
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
