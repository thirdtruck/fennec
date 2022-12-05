use serde::{Serialize, Deserialize};

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlyphEditor {
    pub glyph: Glyph,
}

impl GlyphEditor {
    pub fn with_segment_toggled(self, segment: usize) -> GlyphEditor {
        Self {
            glyph: self.glyph.with_toggled_segment(segment),
        }
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleSegmentOnActiveGlyph(segment) => self.with_segment_toggled(segment),
            _ => self
        }
    }
}
