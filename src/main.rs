use std::collections::{HashMap};
use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Debug, Default, PartialEq)]
struct State {
    tick_count: usize,
    glyph_editor: GlyphEditor,
    word_editor: WordEditor,
    all_words: HashMap<usize, Word>,
    all_snippets: HashMap<usize, Snippet>,
}

#[derive(Clone, Debug)]
struct GlyphMap {
    width: usize,
    height: usize,
    glyphs: Vec<Option<Glyph>>,
}

impl GlyphMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            glyphs: vec![None; width * height],
        }
    }

    fn set_glyph(&mut self, x: usize, y: usize, glyph: Glyph) {
        self.glyphs[x + (y * self.width)] = Some(glyph);
    }

    fn get_glyph(&self, x: usize, y: usize) -> Option<Glyph> {
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
                    if glyph.includes_segment(segment) {
                        ctx.set(x + gx, y + gy, PURPLE, TRANSPARENT, segment);
                    }
                }
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.tick_count += 1;

        if let Some(key) = ctx.key {
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
                self.glyph_editor.toggle_segment(segment);
            }
        }

        let mut map = GlyphMap::new(10, 10);
        map.set_glyph(0, 0, Glyph(0b0000_0010_0000_0001));
        map.set_glyph(0, 0, Glyph(0b1111_1111_1111_1110));
        map.set_glyph(1, 0, Glyph(18));
        map.set_glyph(2, 0, Glyph(99));

        let segment_index: usize = (self.tick_count / 3) % 16;

        let mask = Glyph::mask_from_usize(segment_index);
        map.set_glyph(1, 1, (ALL_SEGMENTS ^ mask).into());

        let all_segments: usize = ALL_SEGMENTS.into();
        let glyph_index: usize = (self.tick_count) % all_segments;
        map.set_glyph(3, 1, glyph_index.into());

        self.glyph_editor.apply_active_glyph(|glyph| map.set_glyph(3, 3, glyph));

        draw_map_at(&map, ctx, 1, 1);

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn example_language_usage() {
    let glyph_code: u16 = 0xAF;
    let glyph: Glyph = glyph_code.into();
    let word1 = Word::Tunic(vec![Rc::new(RefCell::new(glyph))]);
    let word2 = Word::English("Testing".into());
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

    main_loop(context, State::default())
}
