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

        let drawing = GlyphDrawing { glyph, color };

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

                        match glyph.includes_segment(segment) {
                            Ok(included) => {
                                if included {
                                    ctx.set(x + gx, y + gy, color, TRANSPARENT, segment)
                                }
                            }
                            Err(error) => { dbg!(error); },
                        }
                    }
                }
            }
        }
    }

    pub fn render_snippet_on(&mut self, view: &SnippetView, x: usize, y: usize) {
        for (index, word_view) in view.word_views.iter().enumerate() {
            self.render_word_on(word_view, x, y + index);
        }
    }

    pub fn render_word_on(&mut self, view: &WordView, x: usize, y: usize) {
        for (index, glyph_view) in view.glyph_views.iter().enumerate() {
            self.render_glyph_on(glyph_view, x + index, y);
        }

        let state_x = 0;
        let state_y = 0;

        if view.selected {
            let color = match &view.state {
                WordEditorState::ModifyGlyphSet => BLUE,
                WordEditorState::ModifySelectedGlyph => YELLOW,
            };

            self.set_glyph(state_x, state_y, Glyph(u16::MAX), color.into());
        }
    }

    pub fn render_glyph_on(&mut self, view: &GlyphView, x: usize, y: usize) {
        let color = if view.selected { YELLOW } else { WHITE };

        self.set_glyph(x, y, view.glyph, color.into());
    }
}
