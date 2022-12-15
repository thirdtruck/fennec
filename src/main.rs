mod editors;
mod gui;
mod language;
mod renderers;
mod views;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::editors::events::*;
    pub use crate::editors::file_editors::*;
    pub use crate::editors::glyph_editors::*;
    pub use crate::editors::notebook_editors::*;
    pub use crate::editors::snippet_editors::*;
    pub use crate::editors::word_editors::*;
    pub use crate::editors::*;
    pub use crate::gui::*;
    pub use crate::language::glyphs::*;
    pub use crate::language::notebooks::*;
    pub use crate::language::snippets::*;
    pub use crate::language::words::*;
    pub use crate::language::*;
    pub use crate::renderers::file_editor_renderers::*;
    pub use crate::renderers::glyph_map_renderers::*;
    pub use crate::renderers::snippet_source_renderers::*;
    pub use crate::renderers::*;
    pub use crate::views::*;

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
}

use prelude::*;

struct State {
    file_editor: FileEditor,
}

impl State {
    fn new(snippet: Snippet) -> Self {
        let notebook: Notebook = vec![snippet].into();
        let file_editor = FileEditor::new(notebook.clone(), DEFAULT_NOTEBOOK_FILE);

        Self { file_editor }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut map = GlyphMap::new(10, 10);

        let ctx_clone = ctx.clone();

        self.file_editor = {
            let file_editor = self.file_editor.clone();

            let event = file_editor.on_input(Box::new(move |editor| {
                on_file_editor_input(editor, &ctx_clone)
            }));

            if event != EditorEvent::NoOp {
                file_editor.apply(event)
            } else {
                file_editor
            }
        };

        self.file_editor.render_with(|file_editor_view| {
            let notebook_view = &file_editor_view.notebook_view;

            map.render_notebook_on(notebook_view, 1, 1);

            let selected_snippet_view = notebook_view
                .snippet_views
                .iter()
                .find(|snippet_view| snippet_view.selected);

            if let Some(snippet_view) = selected_snippet_view {
                render_snippet_source_on(
                    &snippet_view,
                    ctx,
                    1,
                    (SCREEN_HEIGHT - 2).try_into().unwrap(),
                );
            }

            render_file_editor_view_onto(&file_editor_view, ctx);
        });

        map.draw_on(ctx, 1, 1);

        // TODO: Auto-save to a backup file if this encounters an error
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let glyph_font = "tunic_glyphs.png";
    let small_text_font = "dbyte_1x.png";
    let large_text_font = "dbyte_2x.png";

    let starting_snippet: Snippet =
        vec![vec![0xAF, 0x13, 0xFF].into(), vec![0x03, 0x55, 0x78].into()].into();
    let starting_snippet = Snippet {
        source: Some(Source::Other("Example snippet".into())),
        ..starting_snippet
    };

    let state = State::new(starting_snippet);

    let output = serde_yaml::to_string(&state.file_editor.to_view().notebook_view).unwrap();
    println!("Output: {}", output);

    let context = BTermBuilder::new()
        .with_title("Tunic Language Toolkit")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(24, 32)
        .with_resource_path("resources/")
        .with_font(glyph_font, 24, 32)
        .with_font(small_text_font, 6, 8)
        .with_font(large_text_font, 12, 16)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font) // 0
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font) // 14
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, glyph_font) // 15
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, small_text_font) // 16
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, large_text_font) // 17
        .build()?;

    main_loop(context, state)
}
