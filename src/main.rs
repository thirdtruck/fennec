mod editors;
mod gui;
mod language;
mod renderers;
mod views;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::editors::events::*;
    pub use crate::editors::glyph_editors::*;
    pub use crate::editors::notebook_editors::*;
    pub use crate::editors::snippet_editors::*;
    pub use crate::editors::word_editors::*;
    pub use crate::editors::*;
    pub use crate::gui::*;
    pub use crate::language::notebook::*;
    pub use crate::language::*;
    pub use crate::renderers::*;
    pub use crate::renderers::glyph_map_renderers::*;
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
}

use prelude::*;

#[derive(Default)]
struct State {
    tick_count: usize,
    notebook_editor: NotebookEditor,
}

impl State {
    fn new(snippet: Snippet) -> Self {
        let notebook: Notebook = vec![snippet].into();
        let notebook_editor = NotebookEditor::new(notebook).with_snippet_selected(0);

        Self {
            notebook_editor,
            ..Self::default()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick_count += 1;

        let mut map = GlyphMap::new(10, 10);

        let ctx_clone = ctx.clone();

        let editor = self.notebook_editor.clone();

        if let Some(key) = &ctx_clone.key {
            match key {
                VirtualKeyCode::F2 => {
                    self.notebook_editor.render_with(|view, _index| {
                        let output = serde_yaml::to_string(&view).unwrap();
                        println!("YAML output: {}", output);
                    });
                }
                _ => (),
            }
        }

        let event = editor.on_input(Box::new(move |editor| {
            on_notebook_editor_input(editor, &ctx_clone)
        }));

        if event != EditorEvent::NoOp {
            let editor = editor.apply(event);

            self.notebook_editor = editor;
        }

        self.notebook_editor
            .render_with(|view, _index| map.render_notebook_on(&view, 1, 1));

        self.notebook_editor.render_with(|view, _index| {
            let selected_snippet_view = view.snippet_views
                .iter()
                .find(|snippet_view| snippet_view.selected);

            if let Some(snippet_view) = selected_snippet_view {
                let source_text: String = if let Some(source) = &snippet_view.snippet.source {
                    match source {
                        Source::ManualPageNumber(number) => format!("Manual: Page {}", number),
                        Source::ScreenshotFilename(filename) => format!("Screenshot: {}", filename), 
                        Source::Other(string) => format!("Other: {}", string),
                    }
                } else {
                    "(Unknown)".into()
                };

                let source_text = format!("Source -> {}", source_text);

                ctx.set_active_console(16);
                ctx.cls();
                ctx.print_color(1, SCREEN_HEIGHT - 2, WHITE, BLACK, source_text);
            }
        });

        map.draw_on(ctx, 1, 1);

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

    let output = serde_yaml::to_string(&state.notebook_editor).unwrap();
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
