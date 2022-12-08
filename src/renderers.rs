use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct GlyphDrawing {
    pub glyph: Glyph,
    pub color: RGBA,
}

#[derive(Clone, Debug)]
pub struct GlyphMap {
    pub width: usize,
    pub height: usize,
    pub glyphs: Vec<Option<GlyphDrawing>>,
}

impl GlyphMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            glyphs: vec![None; width * height],
        }
    }

    pub fn set_glyph(&mut self, x: usize, y: usize, glyph: Glyph, color: RGBA) {
        let index = x + (y * self.width);

        let drawing = GlyphDrawing {
            glyph,
            color,
        };

        self.glyphs[index] = Some(drawing);
    }

    pub fn get_glyph(&self, x: usize, y: usize) -> Option<GlyphDrawing> {
        *self.glyphs.get(x + (y * self.width)).unwrap()
    }

    pub fn draw_on(&self, ctx: &mut BTerm, x: usize, y: usize) {
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
}
