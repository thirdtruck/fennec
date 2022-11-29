use std::collections::{HashMap};

mod language;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::language::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

const TRANSPARENT: RGBA = RGBA { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

use prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
struct State {
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

    fn draw_glyphs_at(&self, ctx: &mut BTerm, x: usize, y: usize) {
        for segment in 0..15 {
            ctx.set_active_console(segment);
            ctx.cls();

            let segment: u16 = match segment.try_into() {
                Ok(seg) => seg,
                Err(err) => panic!("Invalid segment index: {}", err),
            };

            for gx in 0..self.width {
                for gy in 0..self.height {
                    if let Some(glyph) = self.get_glyph(gx, gy) {
                        if glyph.includes_segment(segment) {
                            ctx.set(x + gx, y + gy, PURPLE, TRANSPARENT, segment);
                        }
                    }
                }
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut map = GlyphMap::new(10, 10);
        map.set_glyph(0, 0, Glyph(0b0000_0010_0000_0001));
        map.set_glyph(0, 0, Glyph(0b1111_1111_1111_1110));
        map.set_glyph(1, 0, Glyph(18));
        map.set_glyph(2, 0, Glyph(99));

        map.draw_glyphs_at(ctx, 1, 1);

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn example_language_usage() {
    let glyph: Glyph = (0xAF).into();
    let word1 = Word::Tunic(vec![glyph]);
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

    let context = BTermBuilder::new()
        .with_title("Tunic Language Toolkit")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("tunic-dungeonfont.png", 64, 64)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "tunic-dungeonfont.png")
        .build()?;

    main_loop(context, State::default())
}
