use crate::prelude::*;

pub fn snippet_source_to_label(snippet_view: &SnippetView) -> String {
    if let Some(source) = &snippet_view.snippet.source {
        match source {
            Source::ManualPageNumber(number) => format!("Manual: Page {}", number),
            Source::ScreenshotFilename(filename) => format!("Screenshot: {}", filename),
            Source::Other(string) => format!("Other: {}", string),
        }
    } else {
        "(Unknown)".into()
    }
}

pub fn render_snippet_source_on(snippet_view: &SnippetView, ctx: &mut BTerm, x: usize, y: usize) {
    let source_text = snippet_source_to_label(snippet_view);

    let source_text = format!("Source -> {}", source_text);

    ctx.set_active_console(16);
    ctx.cls();
    ctx.print_color(x, y, WHITE, BLACK, source_text);
}
