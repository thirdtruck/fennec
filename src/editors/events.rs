use serde::{Deserialize, Serialize};

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
    AddNewTunicWordAtCursor,
    AddNewGlyphToTunicWordAtCursor,
    DeleteGlyphAtCursor,
    DeleteWordAtCursor,
}
