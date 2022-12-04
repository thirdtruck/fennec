use std::collections::HashMap;

mod language;
mod editors;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::language::*;
    pub use crate::editors::*;

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
    #[allow(dead_code)]
    all_words: HashMap<usize, Word>,
    #[allow(dead_code)]
    all_snippets: HashMap<usize, Snippet>,
}

impl State {
    fn new(snippet: Snippet) -> Self {
        let mut snippet_editor = SnippetEditor::new(snippet);
        snippet_editor.edit_word_at(0);

        Self {
            snippet_editor,
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

impl GlyphMap {
    fn draw_active_word(&mut self, word: Word) {
        if let Word::Tunic(glyphs) = word {
            for (index, glyph) in glyphs.iter().enumerate() {
                self.set_glyph(1 + index, 1, *glyph.borrow(), WHITE.into());
            }
        }
    }

    fn draw_selected_glyph(&mut self, selection: GlyphSelection) {
        let glyph = *selection.glyph.borrow();

        if let Some(x_offset) = selection.position_in_word {
            self.set_glyph(1 + x_offset, 1, glyph, YELLOW.into());
        }
    }
}

fn on_edit_glyph(_glyph: Glyph, key: Option<VirtualKeyCode>) -> Vec<EditorEvent> {
    let mut events: Vec<EditorEvent> = vec![];

    if let Some(key) = key {
        let segment = match key {
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
            _ => None,
        };

        if let Some(segment) = segment {
            events.push(EditorEvent::ToggleSegmentOnActiveGlyph(segment));
        }

        match key {
            VirtualKeyCode::Left => events.push(EditorEvent::MoveGlyphCursorLeft),
            VirtualKeyCode::Right => events.push(EditorEvent::MoveGlyphCursorRight),
            _ => (),
        }
    }

    events
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick_count += 1;

        let mut map = GlyphMap::new(10, 10);

        let mut ctx = ctx.clone();
        let key = ctx.key;

        let on_edit_glyph = move |glyph| on_edit_glyph(glyph, key);

        let word_editor_callbacks = WordEditorCallbacks {
            on_edit_glyph: Some(Box::new(on_edit_glyph)),
        };

        if let Some(editor) = &self.snippet_editor.word_editor {
            let editor = editor.with_callbacks(word_editor_callbacks);

            editor.apply_active_word(|word| map.draw_active_word(word));

            editor.apply_selected_glyph(|selection| map.draw_selected_glyph(selection));

            self.snippet_editor.word_editor = Some(editor);
        }

        self.snippet_editor.process_all_events();

        draw_map_at(&map, &mut ctx, 1, 1);

        render_draw_buffer(&mut ctx).expect("Render error");
    }
}

fn example_language_usage() {
    let glyph_code: u16 = 0xAF;
    let glyph: Glyph = glyph_code.into();
    let word1: Word = glyph.into();
    let word2: Word = "Testing".into();
    let word3: Word = vec![0x01, 0x11, 0xF1].into();
    let snippet = Snippet {
        words: vec![word1, word2, word3],
        source: Some(Source::Other("Example snippet".into())),
    };

    println!("Hello, world!");
    println!("Here's your glyph! {:?}", snippet);
}

fn main() -> BError {
    example_language_usage();
    let font_file = "tunic-dungeonfont-16x32.png";

    let snippet: Snippet = vec![
        vec![0xAF, 0x13, 0xFF].into(),
        vec![0x01, 0x55, 0x78].into(),
    ].into();

    let state = State::new(snippet);

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
