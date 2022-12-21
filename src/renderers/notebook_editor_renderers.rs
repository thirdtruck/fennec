use std::error::Error;

use crate::prelude::*;

pub fn render_notebook_on(
    notebook_view: &NotebookView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    ctx.set_active_console(17);
    ctx.cls();

    match notebook_view.state {
        NotebookEditorState::SelectingSnippet => {
            let snippet_views: Vec<(usize, SnippetView)> = notebook_view
                .snippet_views
                .iter()
                .enumerate()
                .filter(|(_index, view)| view.transcribed)
                .map(|(index, view)| (index, view.clone()))
                .collect();

            for (relative_index, snippet_view) in snippet_views.iter().enumerate() {
                let (_, snippet_view) = snippet_view;

                let relative_index: u32 = relative_index.try_into()?;

                let y = y + (relative_index * 3);

                let description_label = snippet_view.snippet.description.clone();
                let description_label = if snippet_view.selected {
                    format!("-> {:3}) {}", relative_index, description_label)
                } else {
                    format!("{:6}) {}", relative_index, description_label)
                };

                let source_label = snippet_source_to_label(snippet_view);
                let source_label = format!("        Source: {}", source_label);

                let description_color = description_color_for(&snippet_view);
                let source_color = source_color_for(&snippet_view);

                ctx.print_color(x, y, description_color, BLACK, description_label);
                ctx.print_color(x, y + 1, source_color, BLACK, source_label);
            }
        }
        NotebookEditorState::EditingSnippet => {
            render_selected_snippet_on(notebook_view, map, ctx, x, y)?
        }
    };

    Ok(())
}

fn description_color_for(view: &SnippetView) -> (u8, u8, u8) {
    let SnippetView { selected, transcribed, .. } = view.clone();

    if selected && transcribed {
        YELLOW
    } else if selected {
        GREEN
    } else if transcribed {
        GRAY15
    } else {
        WHITE
    }
}

fn source_color_for(view: &SnippetView) -> (u8, u8, u8) {
    let SnippetView { selected, transcribed, .. } = view.clone();

    if selected && transcribed {
        GRAY20
    } else if selected {
        WHITE
    } else if transcribed {
        GRAY10
    } else {
        WHITE
    }
}

pub fn render_selected_snippet_on(
    notebook_view: &NotebookView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let selected_snippet_view = notebook_view
        .snippet_views
        .iter()
        .find(|snippet_view| snippet_view.selected);

    if let Some(snippet_view) = selected_snippet_view {
        map.render_snippet_on(snippet_view, x, y)?;

        render_snippet_source_on(&snippet_view, ctx, 1, (SCREEN_HEIGHT - 2).try_into()?);
    }

    Ok(())
}
