use crate::prelude::*;

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
