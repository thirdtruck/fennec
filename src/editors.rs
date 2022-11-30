use crate::prelude::*;

use std::rc::Rc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlyphEditor {
    active_glyph: Glyph,
}

impl GlyphEditor {
    pub fn toggle_segment(&mut self, segment: usize) {
        self.active_glyph = self.active_glyph.with_toggled_segment(segment);
    }

    pub fn apply_active_glyph<F>(&self, mut receiver: F)
        where F: FnMut(Glyph)
    {
        receiver(self.active_glyph.clone());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
    active_glyph: Option<Rc<Glyph>>,
}

impl WordEditor {
    pub fn apply_active_glyph<F>(&self, mut receiver: F)
        where F: FnMut(Glyph)
    {
        if let Some(glyph) = self.active_glyph.clone() {
            receiver((*glyph).clone());
        }
    }
}

impl Default for WordEditor {
    fn default() -> Self {
        Self {
            active_word: Word::default(),
            glyph_editor: None,
            active_glyph: None,
        }
    }
}
