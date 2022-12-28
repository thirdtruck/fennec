use std::error::Error;

use crate::prelude::*;

pub fn render_snippet_on(
    snippet_view: &SnippetView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    ctx.set_active_console(SNIPPET_CONSOLE);
    ctx.cls();

    map.render_snippet_on(snippet_view, x, y)?;

    let x_offset: u32 = 3;
    let y_offset: u32 = 3;

    ctx.set_active_console(SNIPPET_CONSOLE);
    ctx.cls();

    let english_word_views: Vec<(usize, &WordView)> = snippet_view
        .word_views
        .iter()
        .enumerate()
        .filter(|(_index, word_view)| if let Word::English(_) = &word_view.word { true } else { false })
        .collect();

    for (index, view) in english_word_views {
        let index: u32 = index.try_into()?;
        if let Word::English(text) = &view.word {
            let x = x + x_offset;
            let y = (index * 2) + y + y_offset;
            ctx.print_color(x, y, ORANGE, BLACK, text);
        }
    }

    Ok(())
}
