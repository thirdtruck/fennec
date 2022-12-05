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
    NoOp,
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
    pub fn with_segment_toggled(self, segment: usize) -> GlyphEditor {
        Self {
            glyph: self.glyph.with_toggled_segment(segment),
            ..self
        }
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleSegmentOnActiveGlyph(segment) => self.with_segment_toggled(segment),
            _ => self
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

#[derive(Clone, Debug)]
pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
    active_glyph_index: Option<usize>,
    //pub callbacks: WordEditorCallbacks,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word.into(),
            glyph_editor: None,
            active_glyph_index: None,
            //callbacks: WordEditorCallbacks::default(),
        }
    }

    /*
    pub fn with_callbacks(&self, callbacks: WordEditorCallbacks) -> Self {
        Self {
            active_word: self.active_word.clone(),
            glyph_editor: self.glyph_editor.clone(),
            active_glyph_index: self.active_glyph_index,
            callbacks,
        }
    }
    */

    pub fn with_glyph_selected(self, index: usize) -> Self {
        let mut glyph_editor = self.glyph_editor.clone();
        let mut active_glyph_index = self.active_glyph_index.clone();

        if let Word::Tunic(glyphs) = &self.active_word {
            if let Some(glyph) = glyphs.get(index) {
                let glyph = glyph.clone();

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

impl Default for WordEditor {
    fn default() -> Self {
        Self {
            active_word: Word::default().into(),
            glyph_editor: None,
            active_glyph_index: None,
            //callbacks: WordEditorCallbacks::default(),
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

#[derive(Clone, Debug)]
pub struct SnippetEditor {
    active_snippet: Snippet,
    pub word_editor: Option<WordEditor>,
    active_word_index: Option<usize>,
    //pub callbacks: SnippetEditorCallbacks,
    //pub word_editor_callbacks: Option<WordEditorCallbacks>,
}

impl SnippetEditor {
    pub fn new(snippet: Snippet) -> Self {
        Self {
            active_snippet: snippet.into(),
            word_editor: None,
            active_word_index: None,
            //callbacks: SnippetEditorCallbacks::default(),
            //word_editor_callbacks: None,
        }
    }

    pub fn on_input(&self, callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent>) -> EditorEvent {
        callback(self)
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

    pub fn apply(self, event: EditorEvent) -> Self {
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
            //callbacks: SnippetEditorCallbacks::default(),
            //word_editor_callbacks: None,
        }
    }
}
