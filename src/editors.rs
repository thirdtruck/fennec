use serde::{Deserialize, Serialize};

pub mod glyph_editors;
pub mod notebook_editors;
pub mod snippet_editors;
pub mod word_editors;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EditorEvent {
    NoOp,
    ToggleSegmentOnSelectedGlyph(Segment),
    MoveGlyphCursorForward,
    MoveGlyphCursorBackward,
    MoveWordCursorForward,
    MoveWordCursorBackward,
    ToggleGlyphEditingMode,
    AddNewTunicWord, // TODO: Add an index property, e.g. AddNewTunicWord(usize)
    AddNewGlyphToTunicWord, // TODO: Add an index property, e.g. AddNewTunicWord(usize)
}
