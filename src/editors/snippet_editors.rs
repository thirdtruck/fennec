use serde::{Deserialize, Serialize};

use std::cmp;

use crate::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SnippetEditor {
    selected_snippet: Snippet,
    #[serde(skip_serializing)]
    word_editor: Option<WordEditor>,
    #[serde(skip_serializing)]
    selected_word_index: Option<usize>,
}

impl SnippetEditor {
    pub fn new(snippet: Snippet) -> Self {
        Self {
            selected_snippet: snippet,
            word_editor: None,
            selected_word_index: None,
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
        let words = self.selected_snippet.words.clone();

        if let Some(word) = words.get(index) {
            let editor = WordEditor::new(word.clone()).with_glyph_selected(0);

            SnippetEditor {
                word_editor: Some(editor),
                selected_word_index: Some(index),
                ..self
            }
        } else {
            self
        }
    }

    pub fn with_word_selection_moved_forward(self, amount: usize) -> Self {
        if let Some(selected_word_index) = self.selected_word_index {
            let index = cmp::min(
                self.selected_snippet.words.len(),
                selected_word_index + amount,
            );
            self.with_word_selected(index)
        } else {
            self
        }
    }

    pub fn with_word_selection_moved_backward(self, amount: usize) -> Self {
        if let Some(selected_word_index) = self.selected_word_index {
            let index = if selected_word_index >= amount {
                selected_word_index - amount
            } else {
                0
            };

            self.with_word_selected(index)
        } else {
            self
        }
    }

    // TODO: Refactor this into with_new_tunic_word_at(index) and move the logic for word placement
    // into the UI layer
    pub fn with_new_tunic_word(self) -> Self {
        let mut words = self.selected_snippet.words.clone();
        let new_word: Word = vec![0x10].into(); // TODO: arbitrary starting value

        let new_index = if let Some(selected_word_index) = self.selected_word_index {
            if selected_word_index + 1 == words.len() {
                words.push(new_word);
            } else {
                words.insert(selected_word_index + 1, new_word);
            }

            selected_word_index + 1
        } else {
            words.push(new_word);

            words.len() - 1
        };

        let selected_snippet = Snippet {
            words,
            ..self.selected_snippet
        };

        Self {
            selected_snippet,
            ..self
        }
        .with_word_selected(new_index)
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::MoveWordCursorBackward => self.with_word_selection_moved_backward(1),
            EditorEvent::MoveWordCursorForward => self.with_word_selection_moved_forward(1),
            EditorEvent::AddNewTunicWord => self.with_new_tunic_word(),
            _ => {
                if let Some(editor) = self.word_editor {
                    let word_editor = editor.apply(event);

                    let mut snippet = self.selected_snippet.clone();

                    if let Some(index) = self.selected_word_index {
                        if let Some(word) = snippet.words.get_mut(index) {
                            *word = word_editor.selected_word();
                        }
                    }

                    Self {
                        selected_snippet: snippet,
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
            .selected_snippet
            .words
            .iter()
            .enumerate()
            .map(|(word_index, word)| {
                let selected = if let Some(selected_word_index) = self.selected_word_index {
                    word_index == selected_word_index
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
            snippet: self.selected_snippet.clone(),
            word_views,
            selected: true,
        };

        renderer(view, 0)
    }
}
