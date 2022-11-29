use std::collections::{HashMap};
use std::convert::From;

const TRANSPARENT: RGBA = RGBA { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Source {
    ManualPageNumber(usize),
    ScreenshotFilename(String),
    Other(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Glyph(u16);

impl From<u16> for Glyph {
    fn from(item: u16) -> Self {
        Self(item)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Word {
    Tunic(Vec<Glyph>),
    English(String),
}

impl From<Vec<u16>> for Word {
    fn from(items: Vec<u16>) -> Self {
        let glyphs: Vec<Glyph> = items
            .iter()
            .map(|g| (*g).into())
            .collect();

        Self::Tunic(glyphs)
    }
}

impl From<&[u16]> for Word {
    fn from(items: &[u16]) -> Self {
        let glyphs: Vec<Glyph> = items
            .iter()
            .map(|g| (*g).into())
            .collect();

        Self::Tunic(glyphs)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Snippet {
    words: Vec<Word>,
    source: Option<Source>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct State {
    all_words: HashMap<usize, Word>,
    all_snippets: HashMap<usize, Snippet>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // let mut draw_batch = DrawBatch::new();

        let x = 12;
        let y = 8;

        for glyph in 0..15 {
            ctx.set_active_console(glyph);
            ctx.cls();

            ctx.set(x, y, BLUE, TRANSPARENT, glyph);
            ctx.set(x+2, y, BLUE, TRANSPARENT, glyph);
        }

        ctx.print_color_centered(
            7,
            WHITE,
            TRANSPARENT,
            "Tunic glyphs below",
        );

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
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

    let context = BTermBuilder::new()
        .with_title("Tunic Language Toolkit")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("tunic-dungeonfont.png", 64, 64)
        //.with_font("dungeonfont.png", 32, 32)
        //.with_font("terminal8x8.png", 8, 8)
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
