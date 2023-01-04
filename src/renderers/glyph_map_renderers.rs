use std::error::Error;
use std::fmt;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct GlyphRenderError {
    description: String,
}

impl GlyphRenderError {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

impl fmt::Display for GlyphRenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GlyphRenderError: {}", self.description)
    }
}

impl Error for GlyphRenderError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GlyphDrawing {
    pub glyph: Glyph,
    pub color: RGBA,
}

#[derive(Clone, Debug)]
pub struct GlyphMap {
    pub width: u32,
    pub height: u32,
    pub glyphs: Vec<Option<GlyphDrawing>>,
}

impl GlyphMap {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let size: usize = (width * height).try_into()?;

        Ok(Self {
            width,
            height,
            glyphs: vec![None; size],
        })
    }

    pub fn set_glyph(
        &mut self,
        x: u32,
        y: u32,
        glyph: Glyph,
        color: RGBA,
    ) -> Result<(), Box<dyn Error>> {
        let index: usize = (x + (y * self.width)).try_into()?;

        let drawing = GlyphDrawing { glyph, color };

        if let Some(glyph) = self.glyphs.get_mut(index) {
            *glyph = Some(drawing);
            Ok(())
        } else {
            let max = self.glyphs.len();
            let err = GlyphRenderError::new(
                format!("Invalid index for the GlyphMap: {}. The limit is {}.", index, max)
            );
            Err(Box::new(err))
        }
    }

    pub fn get_glyph(&self, x: u32, y: u32) -> Option<GlyphDrawing> {
        let index = x + (y * self.width);

        usize::try_from(index)
            .ok()
            .and_then(|idx| self.glyphs.get(idx))
            .and_then(|glyph| *glyph)
    }

    pub fn draw_on(&self, ctx: &mut BTerm, x: u32, y: u32) -> Result<(), Box<dyn Error>> {
        for segment in 0..GLYPH_SEGMENT_COUNT {
            ctx.set_active_console(segment);
            ctx.cls();

            let segment: u16 = segment.try_into().unwrap();

            self.draw_segments_on(ctx, x, y, segment)?;
        }

        Ok(())
    }

    fn draw_segments_on(
        &self,
        ctx: &mut BTerm,
        x: u32,
        y: u32,
        segment: u16,
    ) -> Result<(), Box<dyn Error>> {
        for gx in 0..self.width {
            for gy in 0..self.height {
                let gx: u32 = gx.try_into()?;
                let gy: u32 = gy.try_into()?;

                if let Some(glyph) = self.get_glyph(gx, gy) {
                    let color = glyph.color;
                    let glyph = glyph.glyph;

                    if glyph.includes_segment(segment)? {
                        ctx.set(x + gx, y + gy, color, TRANSPARENT, segment)
                    }
                }
            }
        }

        Ok(())
    }

    pub fn render_snippet_on(
        &mut self,
        view: &SnippetView,
        x: u32,
        y: u32,
    ) -> Result<(), Box<dyn Error>> {
        for (index, word_view) in view.word_views.iter().enumerate() {
            let index: u32 = index.try_into()?;
            self.render_word_on(word_view, x, y + index)?;
        }

        Ok(())
    }

    pub fn render_word_on(
        &mut self,
        view: &WordView,
        x: u32,
        y: u32,
    ) -> Result<(), Box<dyn Error>> {
        for (index, glyph_view) in view.glyph_views.iter().enumerate() {
            let index: u32 = index.try_into()?;

            // Add "word line" that connects glyphs in a word
            let glyph_view = GlyphView {
                glyph: glyph_view.glyph.with_toggled_segment(0)?,
                ..*glyph_view
            };

            self.render_glyph_on(&glyph_view, x + index, y)?;
        }

        let state_x = 0;
        let state_y = 0;

        if view.selected {
            let color = match &view.state {
                WordEditorState::ModifyGlyphSet => BLUE,
                WordEditorState::ModifySelectedGlyph => YELLOW,
            };

            self.set_glyph(state_x, state_y, Glyph(u16::MAX), color.into())?;
        }

        Ok(())
    }

    pub fn render_glyph_on(
        &mut self,
        view: &GlyphView,
        x: u32,
        y: u32,
    ) -> Result<(), Box<dyn Error>> {
        let color = if view.selected { YELLOW } else { WHITE };

        self.set_glyph(x, y, view.glyph, color.into())?;

        Ok(())
    }
}
