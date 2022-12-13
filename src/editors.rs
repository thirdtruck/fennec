pub mod events;
pub mod glyph_editors;
pub mod notebook_editors;
pub mod snippet_editors;
pub mod word_editors;

use crate::prelude::*;

pub trait AppliesEditorEvents {
    fn apply(self, event: EditorEvent) -> Self;
}
