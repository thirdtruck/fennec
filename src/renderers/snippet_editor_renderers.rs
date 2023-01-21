use std::error::Error;

use crate::prelude::*;

#[derive(Clone, Debug)]
struct WordViewIndex {
    view: WordView,
    relative_index: usize,
    absolute_index: usize,
}

pub fn render_snippet_on(
    snippet_view: &SnippetView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let indexed_visible_word_views: Vec<WordViewIndex> = snippet_view
        .word_views
        .iter()
        .enumerate()
        .filter(|(_, view)| view.within_visible_range)
        .enumerate()
        .map(|(absolute_index, (relative_index, view))| WordViewIndex {
            view: view.clone(),
            relative_index,
            absolute_index,
        })
        .collect();

    let word_count: u32 = snippet_view.word_views.len().try_into()?;

    let x_offset: u32 = 3;
    let y_offset: u32 = 3;

    ctx.set_active_console(SNIPPET_CONSOLE);
    ctx.cls();

    for view_index in &indexed_visible_word_views {
        let relative_index: u32 = view_index.relative_index.try_into()?;
        let absolute_index: u32 = view_index.absolute_index.try_into()?;
        let view = &view_index.view;
        let word = &view_index.view.word;

        match &word.word_type {
            WordType::English(word) => {
                let x = x + x_offset;
                let y = (absolute_index * 2) + y + y_offset;
                let color = if view.selected { YELLOW } else { WHITE };

                ctx.print_color(x, y, color, BLACK, word.text());
            }
            WordType::Tunic(_) => map.render_word_on(view, x, y + absolute_index)?,
        };

        let x = 1;
        let y = (absolute_index * 2) + y + y_offset;

        let more_below_index: u32 = MAX_VISIBLE_WORDS.try_into()?;
        let more_below_index = more_below_index - 1;

        let more_below = absolute_index == more_below_index && relative_index < word_count - 1;
        let more_above = absolute_index == 0 && relative_index > 0;

        let snippet_index_text = if more_below {
            "++".to_string()
        } else if more_above {
            "^^".to_string()
        } else {
            relative_index.to_string()
        };

        ctx.print_color(x, y, YELLOW, BLACK, snippet_index_text);
    }

    Ok(())
}
