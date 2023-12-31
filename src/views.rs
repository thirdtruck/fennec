use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlyphView {
    pub glyph: Glyph,
    pub selected: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WordView {
    pub word: Word,
    pub index: usize,
    pub within_visible_range: bool,
    pub selected: bool,
    pub glyph_views: Vec<GlyphView>,
    pub definition: Definition,
}

#[derive(Clone, Debug)]
pub struct WordViewParams {
    pub index: usize,
    pub within_visible_range: bool,
    pub selected: bool,
    pub definition: Definition,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SnippetView {
    pub snippet: Snippet,
    pub selected: bool,
    pub word_views: Vec<WordView>,
    pub transcribed: bool,
    pub retained: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NotebookView {
    pub state: NotebookEditorState,
    pub snippet_views: Vec<SnippetView>,
}

#[derive(Clone, Debug)]
pub struct FileEditorView {
    pub notebook_view: NotebookView,
    pub state: FileEditorState,
    pub target_file: String,
}
