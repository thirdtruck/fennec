use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlyphEditor {
    pub glyph: Glyph,
}

impl GlyphEditor {
    pub fn new(glyph: Glyph) -> Self {
        Self { glyph }
    }

    pub fn with_segment_toggled(self, segment: usize) -> GlyphEditor {
        Self {
            glyph: self.glyph.with_toggled_segment(segment),
        }
    }

    pub fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleSegmentOnSelectedGlyph(segment) => {
                self.with_segment_toggled(segment)
            }
            _ => self,
        }
    }

    pub fn to_view(&self, selected: bool) -> GlyphView {
        GlyphView {
            glyph: self.glyph,
            selected,
        }
    }
}
