mod language;
mod editors;
mod views;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::language::*;
    pub use crate::editors::*;
    pub use crate::editors::glyph_editors::*;
    pub use crate::editors::word_editors::*;
    pub use crate::editors::snippet_editors::*;
    pub use crate::views::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub const TRANSPARENT: RGBA = RGBA { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

use prelude::*;

#[derive(Default)]
struct State {
    tick_count: usize,
    snippet_editor: SnippetEditor,
}

impl State {
    fn new(snippet: Snippet) -> Self {
        Self {
            snippet_editor: SnippetEditor::new(snippet).with_word_selected(0),
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct GlyphDrawing {
    glyph: Glyph,
    color: RGBA,
}

#[derive(Clone, Debug)]
struct GlyphMap {
    width: usize,
    height: usize,
    glyphs: Vec<Option<GlyphDrawing>>,
}

impl GlyphMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            glyphs: vec![None; width * height],
        }
    }

    fn set_glyph(&mut self, x: usize, y: usize, glyph: Glyph, color: RGBA) {
        let index = x + (y * self.width);

        let drawing = GlyphDrawing {
            glyph,
            color,
        };

        self.glyphs[index] = Some(drawing);
    }

    fn get_glyph(&self, x: usize, y: usize) -> Option<GlyphDrawing> {
        *self.glyphs.get(x + (y * self.width)).unwrap()
    }
}

fn draw_map_at(map: &GlyphMap, ctx: &mut BTerm, x: usize, y: usize) {
    for segment in 0..15 {
        ctx.set_active_console(segment);
        ctx.cls();

        let segment: u16 = match segment.try_into() {
            Ok(seg) => seg,
            Err(err) => panic!("Invalid segment index: {}", err),
        };

        for gx in 0..map.width {
            for gy in 0..map.height {
                if let Some(glyph) = map.get_glyph(gx, gy) {
                    let color = glyph.color;
                    let glyph = glyph.glyph;

                    if glyph.includes_segment(segment) {
                        ctx.set(x + gx, y + gy, color, TRANSPARENT, segment);
                    }
                }
            }
        }
    }
}

fn map_key_to_glyph_segment(key: VirtualKeyCode) -> Option<Segment> {
    match key {
        VirtualKeyCode::W => Some(0),
        VirtualKeyCode::E => Some(1),
        VirtualKeyCode::R => Some(2),

        VirtualKeyCode::A => Some(3),
        VirtualKeyCode::S => Some(4),
        VirtualKeyCode::D => Some(5),
        VirtualKeyCode::F => Some(6),

        VirtualKeyCode::U => Some(7),
        VirtualKeyCode::I => Some(8),
        VirtualKeyCode::O => Some(9),
        VirtualKeyCode::P => Some(10),

        VirtualKeyCode::J => Some(11),
        VirtualKeyCode::K => Some(12),
        VirtualKeyCode::L => Some(13),
        VirtualKeyCode::Semicolon => Some(14),
        VirtualKeyCode::Q => Some(15),

        _ => None
    }
}

fn on_modify_selected_glyph(_editor: &GlyphEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        if let Some(segment) = map_key_to_glyph_segment(key) {
            EditorEvent::ToggleSegmentOnActiveGlyph(segment)
        } else {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,

                _ => EditorEvent::NoOp
            }
        }

    } else {
        EditorEvent::NoOp
    }
}

fn on_modify_glyph_set(_editor: &WordEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        if let Some(segment) = map_key_to_glyph_segment(key) {
            EditorEvent::ToggleSegmentOnActiveGlyph(segment)
        } else {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,

                _ => EditorEvent::NoOp
            }
        }

    } else {
        EditorEvent::NoOp
    }
}

fn on_editor_input(editor: &SnippetEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        match key {
            VirtualKeyCode::Up => EditorEvent::MoveWordCursorBackward,
            VirtualKeyCode::Down => EditorEvent::MoveWordCursorForward,
            VirtualKeyCode::Q => EditorEvent::ToggleGlyphEditingMode,

            _ => {
                let callbacks = WordEditorCallbacks {
                    on_modify_selected_glyph: Box::new(move |glyph_editor| on_modify_selected_glyph(glyph_editor, Some(key))),
                    on_modify_glyph_set: Box::new(move |word_editor| on_modify_glyph_set(word_editor, Some(key))),
                };

                editor.on_word_editor_input(callbacks)
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick_count += 1;

        let mut map = GlyphMap::new(10, 10);

        let mut ctx = ctx.clone();
        let key = ctx.key;

        let editor = self.snippet_editor.clone();

        let event = editor.on_input(Box::new(move |editor| on_editor_input(editor, key)));

        if event != EditorEvent::NoOp {
            let editor = editor.apply(event);

            self.snippet_editor = editor;
        }

        self.snippet_editor.render_with(|view, _index| render_snippet(&mut map, &view, 1, 1));

        draw_map_at(&map, &mut ctx, 1, 1);

        ctx.set_active_console(16);
        ctx.cls();
        ctx.print_color(1, 10, WHITE, BLACK, "This is a test of the new font!");

        ctx.set_active_console(17);
        ctx.cls();
        ctx.print_color(1, 11, WHITE, BLACK, "This is a test of the other new font!");

        render_draw_buffer(&mut ctx).expect("Render error");
    }
}

fn render_snippet(map: &mut GlyphMap, view: &SnippetView, x: usize, y: usize) {
    for (index, word_view) in view.word_views.iter().enumerate() {
        render_word(map, word_view, x, y + index);
    }
}

fn render_word(map: &mut GlyphMap, view: &WordView, x: usize, y: usize) {
    for (index, glyph_view) in view.glyph_views.iter().enumerate() {
        render_glyph(map, glyph_view, x + index, y);
    }

    let state_x = 0;
    let state_y = 0;

    if view.selected {
        let color = match &view.state {
            WordEditorState::ModifyGlyphSet => BLUE,
            WordEditorState::ModifySelectedGlyph => YELLOW,
        };

        map.set_glyph(state_x, state_y, Glyph(u16::MAX), color.into());
    }
}

fn render_glyph(map: &mut GlyphMap, view: &GlyphView, x: usize, y: usize) {
    let color = if view.selected {
        YELLOW
    } else {
        WHITE
    };

    map.set_glyph(x, y, view.glyph, color.into());
}

fn example_language_usage() {
    let glyph_code: u16 = 0xAF;
    let glyph: Glyph = glyph_code.into();
    let word1: Word = glyph.into();
    let word2: Word = "Testing".into();
    let word3: Word = vec![0x01, 0x11, 0xF1].into();
    let mut snippet: Snippet = vec![word1, word2, word3].into();
    snippet.source = Some(Source::Other("Example snippet".into()));

    println!("Hello, world!");
    println!("Here's your glyph! {:?}", snippet);
}

fn main() -> BError {
    example_language_usage();
    let glyph_font = "tunic_glyphs.png";
    let small_text_font = "dbyte_1x.png";
    let large_text_font = "dbyte_2x.png";

    let snippet: Snippet = vec![
        vec![0xAF, 0x13, 0xFF].into(),
        vec![0x03, 0x55, 0x78].into(),
    ].into();

    let state = State::new(snippet);

    let output = serde_json::to_string(&state.snippet_editor).unwrap();
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
        .with_simple_console_no_bg(DISPLAY_WIDTH*2, DISPLAY_HEIGHT*2, small_text_font) // 16
        .with_simple_console_no_bg(DISPLAY_WIDTH*2, DISPLAY_HEIGHT*2, large_text_font) // 17
        .build()?;

    main_loop(context, state)
}
