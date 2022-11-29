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

fn set_glyph_color(ctx: &mut BTerm, x: usize, y: usize, color: (u8, u8, u8), glyph: Glyph) {
    for console in 0..15 {
        if (glyph.0 & console) > 0 {
            ctx.set_active_console(console.into());

            ctx.set(x, y, color, TRANSPARENT, glyph.0);
         }
    }
}

fn set_glyph(ctx: &mut BTerm, x: usize, y: usize, glyph: Glyph) {
    set_glyph_color(ctx, x, y, WHITE, glyph);
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

    fn draw_glyph_at(&self, ctx: &mut BTerm, x: usize, y: usize) {
        for segment in 0..15 {
            ctx.set_active_console(segment);
            ctx.cls();

            if let Some(glyph) = self.get_glyph(x, y) {
                let mask: u16 = match segment {
                    00 => 0b1000_0000_0000_0000,
                    01 => 0b0100_0000_0000_0000,
                    02 => 0b0010_0000_0000_0000,
                    03 => 0b0001_0000_0000_0000,
                    04 => 0b0000_1000_0000_0000,
                    05 => 0b0000_0100_0000_0000,
                    06 => 0b0000_0010_0000_0000,
                    07 => 0b0000_0001_0000_0000,
                    08 => 0b0000_0000_1000_0000,
                    09 => 0b0000_0000_0100_0000,
                    10 => 0b0000_0000_0010_0000,
                    11 => 0b0000_0000_0001_0000,
                    12 => 0b0000_0000_0000_1000,
                    13 => 0b0000_0000_0000_0100,
                    14 => 0b0000_0000_0000_0010,
                    15 => 0b0000_0000_0000_0001,
                    _ => panic!("Unexpected segment index: {}", segment),
                };
                if (mask & glyph.0) > 0 {
                    ctx.set(1, 1, PURPLE, TRANSPARENT, segment);
                }
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // let mut draw_batch = DrawBatch::new();

        let mut map = GlyphMap::new(10, 10);
        map.set_glyph(0, 0, Glyph(0b0000_0010_0000_0001));
        map.set_glyph(0, 0, Glyph(0b1111_1111_1111_1110));
        map.set_glyph(1, 0, Glyph(18));
        map.set_glyph(2, 0, Glyph(99));

        let x = 4;
        let y = 0;

        for glyph in 0..15 {
            ctx.set_active_console(glyph);
            ctx.cls();

            ctx.set(x, y, BLUE, TRANSPARENT, glyph);
            ctx.set(x+2, y, BLUE, TRANSPARENT, glyph);
        }

        let x = 22;
        let y = 1;

        for glyph in 0..15 {
            ctx.set_active_console(glyph);
            ctx.cls();

            ctx.set(x, y, ORANGE, TRANSPARENT, glyph);
            ctx.set(x+2, y, ORANGE, TRANSPARENT, glyph);
        }

        ctx.print_color_centered(
            2,
            WHITE,
            TRANSPARENT,
            "Tunic glyphs below",
        );

        let x = 4;
        let y = 3;

        ctx.print_color(x, y, WHITE, TRANSPARENT, "Loop");

        for dash in 0..15 {
            set_glyph_color(ctx, x, y+1, BLUE, Glyph(dash));
        }

        let x = 4;
        let y = 5;

        ctx.print_color(x, y, WHITE, TRANSPARENT, "individual");

        set_glyph_color(ctx, x, y+1,GREEN, Glyph(3));
        set_glyph_color(ctx, x, y+1,GREEN, Glyph(4));
        set_glyph_color(ctx, x, y+1,GREEN, Glyph(5));
        set_glyph_color(ctx, x, y+1,GREEN, Glyph(6));
        set_glyph_color(ctx, x, y+1,GREEN, Glyph(7));

        let x = 4;
        let y = 7;

        ctx.print_color(x, y, WHITE, TRANSPARENT, "Larger numbers");

        set_glyph_color(ctx, x, y+1,YELLOW, Glyph(12));
        set_glyph_color(ctx, x+1, y+1,YELLOW, Glyph(13));

        let x = 4;
        let y = 9;

        ctx.print_color(x, y, WHITE, TRANSPARENT, "Seems to work...?");

        set_glyph_color(ctx, x, y+1, RED, Glyph(14));
        set_glyph_color(ctx, x, y+1, RED, Glyph(11));
        set_glyph_color(ctx, x, y+1, RED, Glyph(9));

        let x = 4;
        let y = 11;

        ctx.print_color(x, y, WHITE, TRANSPARENT, "All down the line");

        for glyph in 0..15 {
            if let Ok(g) = glyph.try_into() {
                set_glyph_color(ctx, x+glyph, y+1, RED, Glyph(g));
            }
        }

        let x = 4;
        let y = 13;

        ctx.print_color(x, y, WHITE, TRANSPARENT, "Combined in one");

        for glyph in 0..3 {
            if let Ok(g) = glyph.try_into() {
                set_glyph_color(ctx, x, y+1, RED, Glyph(g));
            }
        }

        for glyph in 4..7 {
            if let Ok(g) = glyph.try_into() {
                set_glyph_color(ctx, x+1, y+1, RED, Glyph(g));
            }
        }

        for glyph in 8..11 {
            if let Ok(g) = glyph.try_into() {
                set_glyph_color(ctx, x+2, y+1, RED, Glyph(g));
            }
        }

        for glyph in 12..15 {
            if let Ok(g) = glyph.try_into() {
                set_glyph_color(ctx, x+3, y+1, RED, Glyph(g));
            }
        }

        //dbg!(map);

        map.draw_glyph_at(ctx, 0, 0);

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
