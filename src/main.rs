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

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick_count += 1;

        let mut map = GlyphMap::new(10, 10);

        let mut ctx = ctx.clone();
        let key = ctx.key;

        let callback = move |_editor: &SnippetEditor| {
            if let Some(key) = key {
                match key {
                    VirtualKeyCode::W => EditorEvent::ToggleSegmentOnActiveGlyph(0),
                    VirtualKeyCode::E => EditorEvent::ToggleSegmentOnActiveGlyph(1),
                    VirtualKeyCode::R => EditorEvent::ToggleSegmentOnActiveGlyph(2),

                    VirtualKeyCode::A => EditorEvent::ToggleSegmentOnActiveGlyph(3),
                    VirtualKeyCode::S => EditorEvent::ToggleSegmentOnActiveGlyph(4),
                    VirtualKeyCode::D => EditorEvent::ToggleSegmentOnActiveGlyph(5),
                    VirtualKeyCode::F => EditorEvent::ToggleSegmentOnActiveGlyph(6),

                    VirtualKeyCode::U => EditorEvent::ToggleSegmentOnActiveGlyph(7),
                    VirtualKeyCode::I => EditorEvent::ToggleSegmentOnActiveGlyph(8),
                    VirtualKeyCode::O => EditorEvent::ToggleSegmentOnActiveGlyph(9),
                    VirtualKeyCode::P => EditorEvent::ToggleSegmentOnActiveGlyph(10),

                    VirtualKeyCode::J => EditorEvent::ToggleSegmentOnActiveGlyph(11),
                    VirtualKeyCode::K => EditorEvent::ToggleSegmentOnActiveGlyph(12),
                    VirtualKeyCode::L => EditorEvent::ToggleSegmentOnActiveGlyph(13),
                    VirtualKeyCode::Semicolon => EditorEvent::ToggleSegmentOnActiveGlyph(14),
                    VirtualKeyCode::Q => EditorEvent::ToggleSegmentOnActiveGlyph(15),

                    VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorLeft,
                    VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorRight,

                    VirtualKeyCode::Up => EditorEvent::MoveWordCursorLeft,
                    VirtualKeyCode::Down => EditorEvent::MoveWordCursorRight,

                    _ => EditorEvent::NoOp,
                }
            } else {
                EditorEvent::NoOp
            }
        };

        let editor = self.snippet_editor.clone();

        let event = editor.on_input(Box::new(callback));

        if event != EditorEvent::NoOp {
            let editor = editor.apply(event);

            self.snippet_editor = editor;
        }

        self.snippet_editor.render_with(|view, _index| render_snippet(&mut map, &view, 1, 1));

        draw_map_at(&map, &mut ctx, 1, 1);

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
    let font_file = "tunic-dungeonfont-16x32.png";

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
        .with_tile_dimensions(16, 32)
        .with_resource_path("resources/")
        .with_font(font_file, 32, 64)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, font_file)
        .build()?;

    main_loop(context, state)
}
