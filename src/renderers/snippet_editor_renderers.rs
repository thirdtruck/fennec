use std::error::Error;

use crate::prelude::*;

#[derive(Clone, Debug)]
struct WordViewIndex {
    index: usize,
    view: WordView,
}

pub fn render_snippet_on(
    snippet_view: &SnippetView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {

    let indexed_word_views: Vec<WordViewIndex> = snippet_view
        .word_views
        .iter()
        .enumerate()
        .map(|(index, view)| WordViewIndex { index, view: view.clone() })
        .collect();

    let visible_word_views: Vec<WordViewIndex> = indexed_word_views
        .get(snippet_view.word_view_range.clone())
        .map_or(vec![], |views| views.to_vec())
        .iter()
        .map(|view_index| view_index.clone())
        .collect();

    let x_offset: u32 = 3;
    let y_offset: u32 = 3;

    ctx.set_active_console(SNIPPET_CONSOLE);
    ctx.cls();

    for WordViewIndex { index, view } in &visible_word_views {
        let index: u32 = (*index).try_into()?;
        match &view.word {
            Word::English(text) => {
                let x = x + x_offset;
                let y = (index * 2) + y + y_offset;
                let color = if view.selected { YELLOW } else { WHITE };

                ctx.print_color(x, y, color, BLACK, text);
            }
            Word::Tunic(_glyphs) => {
                map.render_word_on(&view, x, y + index)?
            }
        };
    }

    for (index, _view) in visible_word_views.iter().enumerate() {
        let index: u32 = index.try_into()?;
        let x = 1;
        let y = (index * 2) + y + y_offset;
        ctx.print_color(x, y, YELLOW, BLACK, index.to_string());
    }

    Ok(())
}
