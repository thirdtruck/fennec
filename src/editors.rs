use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GlyphEditor {
    pub active_glyph: Glyph,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WordEditor {
    pub active_word: Word,
}
