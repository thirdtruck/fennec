use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EditorEvent {
    ToggleSegmentOnActiveGlyph(Segment),
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

    pub fn apply_active_glyph<F>(&self, mut receiver: F)
        where F: FnMut(Glyph)
    {
        receiver(self.active_glyph.borrow().clone());
    }
}

#[derive(Default)]
pub struct WordEditorCallbacks {
    pub while_editing_glyph: Option<Box<dyn FnMut(Glyph) -> Vec<EditorEvent>>>,
}

pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
    pub callbacks: WordEditorCallbacks,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word,
            glyph_editor: None,
            callbacks: WordEditorCallbacks::default(),
        }
    }

    pub fn edit_glyph_at(&mut self, index: usize) {
        if let Word::Tunic(glyphs) = &self.active_word {
            if let Some(glyph) = glyphs.get(index) {
                self.glyph_editor = Some(GlyphEditor {
                    active_glyph: glyph.clone(),
                    event_queue: vec![],
                });
            }
        }
    }

    pub fn apply_active_glyph<F>(&self, receiver: F)
        where F: FnMut(Glyph)
    {
        if let Some(glyph_editor) = &self.glyph_editor {
            glyph_editor.apply_active_glyph(receiver);
        }
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
            }
        }
    }
}

impl Default for WordEditor {
    fn default() -> Self {
        Self {
            active_word: Word::default(),
            glyph_editor: None,
            callbacks: WordEditorCallbacks::default(),
        }
    }
}
