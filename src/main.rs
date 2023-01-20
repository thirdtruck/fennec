mod cursors;
mod editors;
mod fennec_state;
mod gui;
mod language;
mod renderers;
mod views;
mod visibility;

pub mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::cursors::*;
    pub use crate::editors::events::*;
    pub use crate::editors::file_editors::*;
    pub use crate::editors::glyph_editors::*;
    pub use crate::editors::notebook_editors::*;
    pub use crate::editors::snippet_editors::*;
    pub use crate::editors::word_editors::english_word_editors::*;
    pub use crate::editors::word_editors::tunic_word_editors::*;
    pub use crate::editors::word_editors::*;
    pub use crate::editors::*;
    pub use crate::fennec_state::*;
    pub use crate::gui::*;
    pub use crate::language::dictionary::*;
    pub use crate::language::glyphs::*;
    pub use crate::language::notebooks::*;
    pub use crate::language::snippets::*;
    pub use crate::language::words::*;
    pub use crate::language::*;
    pub use crate::renderers::file_editor_renderers::*;
    pub use crate::renderers::glyph_map_renderers::*;
    pub use crate::renderers::notebook_editor_renderers::*;
    pub use crate::renderers::snippet_editor_renderers::*;
    pub use crate::renderers::*;
    pub use crate::views::*;
    pub use crate::visibility::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub const TRANSPARENT: RGBA = RGBA {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub static DEFAULT_NOTEBOOK_FILE: &str = "notebook.yaml";
    pub static DEFAULT_DICTIONARY_FILE: &str = "dictionary.yaml";

    pub const DEFAULT_GLYPH: Glyph = Glyph(0);

    pub const GLYPH_SEGMENT_COUNT: usize = 16;
    pub const FILE_CONSOLE: usize = 16;
    pub const NOTEBOOK_CONSOLE: usize = 17;
    pub const SNIPPET_CONSOLE: usize = 18;

    pub const MAX_VISIBLE_WORDS: usize = 20;
}
