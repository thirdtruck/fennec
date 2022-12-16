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

    pub fn with_segment_toggled(self, segment: usize) -> Result<GlyphEditor, GlyphError> {
        match self.glyph.with_toggled_segment(segment) {
            Ok(glyph) => Ok(Self { glyph }),
            Err(error) => Err(error),
        }
    }

    pub fn to_view(&self, selected: bool) -> GlyphView {
        GlyphView {
            glyph: self.glyph,
            selected,
        }
    }
}

impl AppliesEditorEvents for GlyphEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::ToggleSegmentOnSelectedGlyph(segment) => {
                let editor = self.clone();

                match self.with_segment_toggled(segment) {
                    Ok(editor) => editor.clone(),
                    Err(error) => { dbg!(error); editor }
                }
            }
            _ => self.clone(),
        }
    }
}
