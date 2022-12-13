use crate::prelude::*;

    //pub fn render_glyph_on(&mut self, view: &GlyphView, x: usize, y: usize) {
    //pub fn draw_on(&self, ctx: &mut BTerm, x: usize, y: usize) {
pub fn render_snippet_source_on(snippet_view: &SnippetView, ctx: &mut BTerm, x: usize, y: usize) {
    let source_text: String = if let Some(source) = &snippet_view.snippet.source {
        match source {
            Source::ManualPageNumber(number) => format!("Manual: Page {}", number),
            Source::ScreenshotFilename(filename) => format!("Screenshot: {}", filename), 
            Source::Other(string) => format!("Other: {}", string),
        }
    } else {
        "(Unknown)".into()
    };

    let source_text = format!("Source -> {}", source_text);

    ctx.set_active_console(16);
    ctx.cls();
    ctx.print_color(x, y, WHITE, BLACK, source_text);
}
