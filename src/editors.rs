use crate::prelude::*;

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WordEditor {
    pub active_word: Word,
}
