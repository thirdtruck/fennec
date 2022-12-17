use fennec::prelude::*;

struct State {
    file_editor: FileEditor,
}

impl State {
    fn new(snippet: Snippet) -> Self {
        let notebook: Notebook = vec![snippet].into();
        let file_editor = FileEditor::new(notebook.clone(), DEFAULT_NOTEBOOK_FILE);
        let file_editor = file_editor.apply(EditorEvent::ConfirmLoadFromFileRequest);

        Self { file_editor }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut map = GlyphMap::new(10, 10).expect("Invalid map dimensions");

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

            render_notebook_on(notebook_view, &mut map, ctx, 1, 1)
                .expect("Notebook editor rendering error");

            render_file_editor_view_onto(&file_editor_view, ctx)
                .expect("File editor rendering error");
        });

        map.draw_on(ctx, 1, 1).expect("Map drawing error");

        // TODO: Auto-save to a backup file if this encounters an error
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let glyph_font = "tunic_glyphs.png";
    let small_text_font = "dbyte_1x.png";
    let large_text_font = "dbyte_2x.png";

    let state = State::new(Snippet::starting_snippet());

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
