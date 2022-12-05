use std::cmp;

use crate::prelude::*;

/*
pub enum GlyphView {
    SelectedGlyph(Glyph),
    PlainGlyph(Glyph),
}
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditorEvent {
    ToggleSegmentOnActiveGlyph(Segment),
    MoveGlyphCursorRight,
    MoveGlyphCursorLeft,
    MoveWordCursorRight,
    MoveWordCursorLeft,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphSelection {
    pub glyph: Glyph,
    pub active: bool,
    pub position_in_word: Option<usize>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GlyphEditor {
    glyph: Glyph,
}

impl GlyphEditor {
    pub fn toggle_segment(&mut self, segment: usize) {
        self.glyph = self.glyph.with_toggled_segment(segment);
    }

    pub fn with_segment_toggled(self, segment: usize) -> GlyphEditor {
        Self {
            glyph: self.glyph.with_toggled_segment(segment),
            ..self
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordSelection {
    pub word: Word,
    pub active: bool,
    pub position_in_snippet: Option<usize>,
}

#[derive(Default)]
pub struct WordEditorCallbacks {
    pub on_edit_glyph: Option<Box<dyn FnMut(Glyph) -> Vec<EditorEvent>>>,
}

pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
    active_glyph_index: Option<usize>,
    pub callbacks: WordEditorCallbacks,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word.into(),
            glyph_editor: None,
            active_glyph_index: None,
            callbacks: WordEditorCallbacks::default(),
        }
    }

    pub fn with_callbacks(&self, callbacks: WordEditorCallbacks) -> Self {
        Self {
            active_word: self.active_word.clone(),
            glyph_editor: self.glyph_editor.clone(),
            active_glyph_index: self.active_glyph_index,
            callbacks,
        }
    }

    /*
    pub fn with_glyph_cursor_moved_forward(self, amount: usize) -> Self {
        let word = self.active_word.clone();

        if let Word::Tunic(glyphs) = word {
            let index = cmp::min(self.active_glyph_index + amount, glyphs.len());

            if self.active_glyph_index + amount
    }
    */

    pub fn edit_glyph_at(&mut self, index: usize) {
        if let Word::Tunic(glyphs) = self.active_word.clone() {
            if let Some(glyph) = glyphs.get(index) {
                self.glyph_editor = Some(GlyphEditor {
                    glyph: glyph.clone(),
                });
                self.active_glyph_index = Some(index);
            }
        }
    }

    pub fn move_glyph_cursor_left(&mut self, amount: usize) {
        let word = self.active_word.clone();

        if let Word::Tunic(_glyphs) = word {
            if let Some(old_index) = self.active_glyph_index {
                let new_index = if old_index >= amount {
                    old_index - amount
                } else {
                    0
                };

                self.edit_glyph_at(new_index);
            }
        }
    }

    pub fn move_glyph_cursor_right(&mut self, amount: usize) {
        let word = self.active_word.clone();

        if let Word::Tunic(glyphs) = word {
            if let Some(old_index) = self.active_glyph_index {
                let new_index = cmp::min(glyphs.len() - 1, old_index + amount);
                self.edit_glyph_at(new_index);
            }
        }
    }

    pub fn toggle_segment_in_active_glyph(&mut self, segment: usize) {
        if let Some(ge) = &mut self.glyph_editor {
            ge.toggle_segment(segment);
        }
    }

    /*
    pub fn apply(&self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleSegmentOnActiveGlyph(segment) => {
                self.toggle_segment_in_active_glyph(segment)
            },
            EditorEvent::MoveGlyphCursorLeft => {
                self.move_glyph_cursor_left(1)
            },
            EditorEvent::MoveGlyphCursorRight => {
                self.move_glyph_cursor_right(1)
            },
            _ => ()
        }
    }
    */

    pub fn process_all_events(&mut self) {
        let mut events: Vec<EditorEvent> = vec![];

        if let Some(editor) = &self.glyph_editor {
            let glyph = editor.glyph.clone();

            let evts = self.callbacks
                .on_edit_glyph
                .as_mut()
                .map(|callback| callback(glyph));

            if let Some(evts) = evts {
                events.extend(evts);
            }
        }

        for evt in events {
            match evt {
                EditorEvent::ToggleSegmentOnActiveGlyph(segment) => {
                    self.toggle_segment_in_active_glyph(segment);
                },
                EditorEvent::MoveGlyphCursorLeft => {
                    self.move_glyph_cursor_left(1)
                },
                EditorEvent::MoveGlyphCursorRight => {
                    self.move_glyph_cursor_right(1)
                },
                _ => ()
            }
        }
    }
}

impl Default for WordEditor {
    fn default() -> Self {
        Self {
            active_word: Word::default().into(),
            glyph_editor: None,
            active_glyph_index: None,
            callbacks: WordEditorCallbacks::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphView {
    pub glyph: Glyph,
    pub selected: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WordView {
    pub word: Word,
    pub selected: bool,
    pub glyph_views: Vec<GlyphView>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnippetView {
    pub snippet: Snippet,
    pub selected: bool,
    pub word_views: Vec<WordView>,
}

#[derive(Default)]
pub struct SnippetEditorCallbacks {
    pub on_edit_word: Option<Box<dyn FnMut(Word) -> Vec<EditorEvent>>>,
}

pub struct SnippetEditor {
    active_snippet: Snippet,
    pub word_editor: Option<WordEditor>,
    active_word_index: Option<usize>,
    pub callbacks: SnippetEditorCallbacks,
    pub word_editor_callbacks: Option<WordEditorCallbacks>,
}

impl SnippetEditor {
    pub fn new(snippet: Snippet) -> Self {
        Self {
            active_snippet: snippet.into(),
            word_editor: None,
            active_word_index: None,
            callbacks: SnippetEditorCallbacks::default(),
            word_editor_callbacks: None,
        }
    }

    pub fn edit_word_at(&mut self, index: usize) {
        let snippet = self.active_snippet.clone();
        if let Some(word) = snippet.words.get(index) {
            let word = word.clone();

            let mut editor = WordEditor::new(word);
            editor.edit_glyph_at(0);

            self.word_editor = Some(editor);
            self.active_word_index = Some(index);
        }
    }

    fn move_word_cursor_left(&mut self, amount: usize) {
        if let Some(index) = self.active_word_index {
            let new_index = if amount > index {
                0
            } else {
                index - amount
            };

            self.edit_word_at(new_index)
        };
    }

    fn move_word_cursor_right(&mut self, amount: usize) {
        let snippet = self.active_snippet.clone();

        if let Some(index) = self.active_word_index {
            let new_index = if (amount + index) < snippet.words.len() {
                0
            } else {
                index - amount
            };

            self.edit_word_at(new_index)
        };
    }

    pub fn process_all_events(&mut self) {
        let mut events: Vec<EditorEvent> = vec![];

        if let Some(editor) = &mut self.word_editor {
            editor.process_all_events();

            let active_word = editor.active_word.clone();

            let evts = self.callbacks
                .on_edit_word
                .as_mut()
                .map(|callback| callback(active_word));

            if let Some(evts) = evts {
                events.extend(evts);
            }
        }

        for evt in events {
            match evt {
                EditorEvent::MoveWordCursorLeft => {
                    self.move_word_cursor_left(1)
                },
                EditorEvent::MoveWordCursorRight => {
                    self.move_word_cursor_right(1)
                },
                _ => (),
            };
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
                                    glyph: glyph.clone(),
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

impl Default for SnippetEditor {
    fn default() -> Self {
        Self {
            active_snippet: Snippet::default().into(),
            word_editor: None,
            active_word_index: None,
            callbacks: SnippetEditorCallbacks::default(),
            word_editor_callbacks: None,
        }
    }
}
