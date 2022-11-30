use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlyphEditor {
    active_glyph: RcGlyph,
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

#[derive(Clone, Debug, PartialEq)]
pub struct WordEditor {
    active_word: Word,
    glyph_editor: Option<GlyphEditor>,
}

impl WordEditor {
    pub fn new(word: Word) -> Self {
        Self {
            active_word: word,
            glyph_editor: None,
        }
    }

    pub fn edit_glyph_at(&mut self, index: usize) {
        if let Word::Tunic(glyphs) = &self.active_word {
            if let Some(glyph) = glyphs.get(index) {
                self.glyph_editor = Some(GlyphEditor { active_glyph: glyph.clone() });
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
}

impl Default for WordEditor {
    fn default() -> Self {
        Self {
            active_word: Word::default(),
            glyph_editor: None,
        }
    }
}
