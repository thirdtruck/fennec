use serde::{Deserialize, Serialize};

use std::cmp;

use crate::prelude::*;

use self::visibility::*;

mod visibility {
    use serde::{Deserialize, Serialize};

    use std::cmp;
    use std::ops::Range;

    use crate::prelude::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct WordViewsVisibility {
        range: Range<usize>,
        max: usize,
    }

    impl WordViewsVisibility {
        pub fn new(views: &Vec<Word>, max: usize) -> Self {
            let start = 0;
            let end = cmp::min(views.len(), max);

            Self {
                range: Range { start, end, },
                max,
            }
        }

        pub fn includes(&self, index: usize) -> bool {
            self.range.contains(&index)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnippetEditor {
    selected_snippet: Snippet,
    word_editor: Option<WordEditor>,
    selected_word_index: Option<usize>,
    visibility: WordViewsVisibility,
}

impl SnippetEditor {
    pub fn new(snippet: Snippet) -> Self {
        let visibility = WordViewsVisibility::new(&snippet.words, MAX_VISIBLE_WORDS);

        Self {
            selected_snippet: snippet,
            word_editor: None,
            selected_word_index: None,
            visibility,
        }
    }

    pub fn selected_snippet(&self) -> Snippet {
        self.selected_snippet.clone()
    }

    pub fn on_input(&self, callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent>) -> EditorEvent {
        callback(self)
    }

    pub fn on_word_editor_input(&self, callbacks: WordEditorCallbacks) -> EditorEvent {
        if let Some(editor) = &self.word_editor {
            // TODO: What if any other conditions should apply here?
            if editor.selected_word().is_empty() {
                EditorEvent::DeleteWordAtCursor
            } else {
                editor.on_input(callbacks)
            }
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

    pub fn with_new_tunic_word_at_cursor(self) -> Self {
        self.with_new_word_at_cursor(vec![DEFAULT_GLYPH].into())
    }

    pub fn with_new_english_word_at_cursor(self) -> Self {
        self.with_new_word_at_cursor(".".into())
    }

    pub fn with_new_word_at_cursor(self, new_word: Word) -> Self {
        let mut words = self.selected_snippet.words.clone();

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

    pub fn with_word_at_cursor_deleted(self) -> Self {
        if let Some(selected_word_index) = self.selected_word_index {
            let mut words = self.selected_snippet.words.clone();

            if words.len() > 0 {
                words.remove(selected_word_index);

                let new_index = if selected_word_index > 0 {
                    selected_word_index - 1
                } else {
                    0
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
            } else {
                self
            }
        } else {
            self
        }
    }

    fn with_transcription_state_toggled(self) -> Self {
        Self {
            selected_snippet: self.selected_snippet.with_transcription_state_toggled(),
            ..self
        }
    }

    fn _with_word_view_slice_moved_forward(self, _amount: usize) -> Self {
        self
    }

    pub fn to_view(&self, selected_snippet: bool, retained: bool) -> SnippetView {
        let word_views: Vec<WordView> = self
            .selected_snippet
            .words
            .iter()
            .enumerate()
            .map(|(word_index, word)| {
                let params = WordViewParams {
                    index: word_index,
                    within_visible_range: self.visibility.includes(word_index),
                    selected: false,
                };

                let selected_word = if let Some(selected_word_index) = self.selected_word_index {
                    word_index == selected_word_index
                } else {
                    false
                };

                if selected_snippet && selected_word {
                    match self.word_editor.as_ref() {
                        Some(editor) => editor.to_view(WordViewParams { selected: true, ..params }),
                        None => {
                            dbg!("Missing WordEditor");
                            dbg!(&self.word_editor);

                            WordEditor::new(word.clone()).to_view(params)
                        }
                    }
                } else {
                    WordEditor::new(word.clone()).to_view(params)
                }
            })
            .collect();

        let transcribed = self.selected_snippet.transcribed;

        SnippetView {
            selected: selected_snippet,
            snippet: self.selected_snippet.clone(),
            word_views,
            transcribed,
            retained,
        }
    }
}

impl AppliesEditorEvents for SnippetEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::MoveWordCursorBackward => self.with_word_selection_moved_backward(1),
            EditorEvent::MoveWordCursorForward => self.with_word_selection_moved_forward(1),
            EditorEvent::AddNewTunicWordAtCursor => self.with_new_tunic_word_at_cursor(),
            EditorEvent::AddNewEnglishWordAtCursor => self.with_new_english_word_at_cursor(),
            EditorEvent::DeleteWordAtCursor => self.with_word_at_cursor_deleted(),
            EditorEvent::ToggleSnippetTranscriptionState => self.with_transcription_state_toggled(),
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
}
