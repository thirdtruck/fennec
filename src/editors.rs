use std::cmp;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EditorEvent {
    ToggleSegmentOnActiveGlyph(Segment),
    MoveGlyphCursorRight,
    MoveGlyphCursorLeft,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphSelection {
    pub glyph: RcGlyph,
    pub active: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
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

    pub fn apply_selected_glyph<F>(&self, mut listener: F)
        where F: FnMut(GlyphSelection)
    {
        let selection = GlyphSelection {
            glyph: self.active_glyph.clone() ,
            active: true,
        };

        listener(selection);
    }

    pub fn apply_active_glyph<F>(&self, mut listener: F)
        where F: FnMut(Glyph)
    {
        listener(self.active_glyph.borrow().clone());
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
            editor.apply_selected_glyph(listener);
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
            let active_glyph = editor.active_glyph.borrow().clone();

            self.callbacks
                .while_editing_glyph
                .as_mut()
                .and_then(|callback| Some(callback(active_glyph)))
                .and_then(|evts| Some(events.extend(evts)));
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
