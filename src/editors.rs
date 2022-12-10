use serde::{Deserialize, Serialize};

pub mod glyph_editors;
pub mod snippet_editors;
pub mod word_editors;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EditorEvent {
    NoOp,
    ToggleSegmentOnActiveGlyph(Segment),
    MoveGlyphCursorForward,
    MoveGlyphCursorBackward,
    MoveWordCursorForward,
    MoveWordCursorBackward,
    ToggleGlyphEditingMode,
}
