use std::cmp;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditorEvent {
    ToggleSegmentOnActiveGlyph(Segment),
    MoveGlyphCursorRight,
    MoveGlyphCursorLeft,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlyphSelection {
    pub glyph: RcGlyph,
    pub active: bool,
    pub position_in_word: Option<usize>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GlyphEditor {
    active_glyph: RcGlyph,
    event_queue: Vec<EditorEvent>,
}

impl GlyphEditor {
    pub fn toggle_segment(&mut self, segment: usize) {
        let mut glyph = self.active_glyph.borrow_mut();
        let toggled_glyph = glyph.with_toggled_segment(segment);

        *glyph = toggled_glyph;
    }

    pub fn apply_selected_glyph<F>(&self, mut listener: F, position_in_word: Option<usize>)
        where F: FnMut(GlyphSelection)
    {
        let selection = GlyphSelection {
            glyph: self.active_glyph.clone() ,
            active: true,
            position_in_word,
        };

        listener(selection);
    }

    #[allow(dead_code)]
    pub fn apply_active_glyph<F>(&self, mut listener: F)
        where F: FnMut(Glyph)
    {
        listener(*self.active_glyph.borrow());
    }
}

#[derive(Default)]
pub struct WordEditorCallbacks {
    pub while_editing_glyph: Option<Box<dyn FnMut(Glyph) -> Vec<EditorEvent>>>,
}

pub struct WordEditor {
    active_word: RcWord,
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

    pub fn edit_glyph_at(&mut self, index: usize) {
        if let Word::Tunic(glyphs) = &self.active_word.borrow().clone() {
            if let Some(glyph) = glyphs.get(index) {
                self.glyph_editor = Some(GlyphEditor {
                    active_glyph: glyph.clone(),
                    event_queue: vec![],
                });
                self.active_glyph_index = Some(index);
            }
        }
    }

    pub fn move_glyph_cursor_left(&mut self, amount: usize) {
        let word = (*self.active_word.borrow()).clone();

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
        let word = (*self.active_word.borrow()).clone();

        if let Word::Tunic(glyphs) = word {
            if let Some(old_index) = self.active_glyph_index {
                let new_index = cmp::min(glyphs.len() - 1, old_index + amount);
                self.edit_glyph_at(new_index);
            }
        }
    }

    #[allow(dead_code)]
    pub fn apply_active_glyph<F>(&self, listener: F)
        where F: FnMut(Glyph)
    {
        if let Some(glyph_editor) = &self.glyph_editor {
            glyph_editor.apply_active_glyph(listener);
        }
    }

    pub fn apply_selected_glyph<F>(&self, listener: F)
        where F: FnMut(GlyphSelection)
    {
        if let Some(editor) = &self.glyph_editor {
            editor.apply_selected_glyph(listener, self.active_glyph_index);
        }
    }

    pub fn apply_active_word<F>(&self, mut listener: F)
        where F: FnMut(Word)
    {
        listener(self.active_word.borrow().clone());
    }

    pub fn toggle_segment_in_active_glyph(&mut self, segment: usize) {
        if let Some(ge) = &mut self.glyph_editor {
            ge.toggle_segment(segment);
        }
    }

    pub fn process_all_events(&mut self) {
        let mut events: Vec<EditorEvent> = vec![];

        if let Some(editor) = &self.glyph_editor {
            let active_glyph = *editor.active_glyph.borrow();

            let evts = self.callbacks
                .while_editing_glyph
                .as_mut()
                .map(|callback| callback(active_glyph));

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
                }
                EditorEvent::MoveGlyphCursorRight => {
                    self.move_glyph_cursor_right(1)
                }
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

#[derive(Default)]
pub struct SnippetEditorCallbacks {
    pub while_editing_word: Option<Box<dyn FnMut(Word) -> Vec<EditorEvent>>>,
}

pub struct SnippetEditor {
    active_snippet: RcSnippet,
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
        let snippet = self.active_snippet.borrow().clone();
        if let Some(word) = snippet.words.get(index) {
            let mut editor = WordEditor::new(word.clone());
            editor.edit_glyph_at(0);

            self.word_editor = Some(editor);
            self.active_word_index = Some(index);
        }
    }

    pub fn process_all_events(&mut self) {
        let mut events: Vec<EditorEvent> = vec![];

        if let Some(editor) = &mut self.word_editor {
            editor.process_all_events();

            let active_word = editor.active_word.borrow().clone();

            let evts = self.callbacks
                .while_editing_word
                .as_mut()
                .map(|callback| callback(active_word));

            if let Some(evts) = evts {
                events.extend(evts);
            }
        }

        /*
        for evt in events {
            match evt {
                _ => (),
            };
        }
        */
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
