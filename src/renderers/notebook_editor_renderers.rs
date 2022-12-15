use crate::prelude::*;

pub fn render_notebook_on(
    notebook_view: &NotebookView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: usize,
    y: usize,
) {
    ctx.set_active_console(17);
    ctx.cls();

    match notebook_view.state {
        NotebookEditorState::SelectingSnippet => {
            for (index, snippet_view) in notebook_view.snippet_views.iter().enumerate() {
                let y = y + index;

                let source_label = snippet_source_to_label(snippet_view);
                let source_label = if snippet_view.selected {
                    format!("-> {:3}) {}", index, source_label)
                } else {
                    format!("{:6}) {}", index, source_label)
                };

                ctx.print_color(x, y, WHITE, BLACK, source_label);
            }
        }
        NotebookEditorState::EditingSnippet => {
            render_selected_snippet_on(notebook_view, map, ctx, x, y)
        }
    }
}

pub fn render_selected_snippet_on(
    notebook_view: &NotebookView,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: usize,
    y: usize,
) {
    let selected_snippet_view = notebook_view
        .snippet_views
        .iter()
        .find(|snippet_view| snippet_view.selected);

    if let Some(snippet_view) = selected_snippet_view {
        map.render_snippet_on(snippet_view, x, y);

        render_snippet_source_on(
            &snippet_view,
            ctx,
            1,
            (SCREEN_HEIGHT - 2).try_into().unwrap(),
        );
    }
}
