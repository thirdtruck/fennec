use std::error::Error;

use crate::prelude::*;

pub fn render_notebook_on(
    notebook_view: &NotebookView,
    dictionary: &Dictionary,
    map: &mut GlyphMap,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    ctx.set_active_console(NOTEBOOK_CONSOLE);
    ctx.cls();

    match notebook_view.state {
        NotebookEditorState::SelectingSnippet => {
            let snippet_views: Vec<(usize, SnippetView)> = notebook_view
                .snippet_views
                .iter()
                .enumerate()
                .filter(|(_index, view)| view.retained)
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
            render_selected_snippet_on(notebook_view, dictionary, map, ctx, x, y)?
        }
    };

    Ok(())
}

fn description_color_for(view: &SnippetView) -> (u8, u8, u8) {
    let SnippetView {
        selected,
        transcribed,
        ..
    } = view.clone();

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
    let SnippetView {
        selected,
        transcribed,
        ..
    } = view.clone();

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
    dictionary: &Dictionary,
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
        let y_from_bottom: u32 = SCREEN_HEIGHT.try_into()?;

        let (selected_word, has_border, colored) = snippet_view
            .word_views
            .iter()
            .filter(|view| view.selected)
            .collect::<Vec<&WordView>>()
            .first()
            .map_or((None, false, false), |view| {
                (Some(view.word.clone()), view.word.has_border(), view.word.colored())
            });

        render_snippet_on(snippet_view, map, ctx, x, y)?;

        if let Some(word) = &selected_word {
            render_selected_word_glyphs_as_base10(word.clone(), ctx, y, y_from_bottom - 7)?;
            render_word_definition(word.clone(), ctx, x, y_from_bottom - 6)?;
        }

        render_description_status(snippet_view, ctx, x, y_from_bottom - 5)?;
        render_source_status(snippet_view, ctx, x, y_from_bottom - 4)?;
        render_transcribed_status(snippet_view, ctx, x, y_from_bottom - 3)?;
        render_has_border_status(has_border, ctx, x, y_from_bottom - 2)?;
        render_colored_status(colored, ctx, x, y_from_bottom - 1)?;
    }

    Ok(())
}
fn format_glyphs_for_reading(glyphs: Vec<Glyph>) -> String {
    glyphs
        .iter()
        .map(|glyph| glyph.0.to_string())
        .reduce(|word, glyph_value| word + " " + &glyph_value)
        .map_or("(Empty)".into(), |word| format!("[{}]", word))
}

fn render_word_definition(
    word: Word,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    let definition = "(pending)".to_string();

    ctx.print_color(x, y, GREEN, BLACK, " Definition:");
    ctx.print_color(x + x_offset, y, WHITE, BLACK, definition);

    Ok(())
}

fn render_selected_word_glyphs_as_base10(
    word: Word,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    match word.word_type {
        WordType::Tunic(tunic_word) => {
            let glyph_values = format_glyphs_for_reading(tunic_word.glyphs());

            ctx.print_color(x, y, GREEN, BLACK, " As Base 10:");
            ctx.print_color(x + x_offset, y, WHITE, BLACK, glyph_values);
        },
        WordType::English(_) => {
            let glyph_values: String = "n/a".into();

            ctx.print_color(x, y, GREEN, BLACK, " As Base 10:");
            ctx.print_color(x + x_offset, y, GRAY40, BLACK, glyph_values);
        },
    };


    Ok(())
}

fn render_description_status(
    snippet_view: &SnippetView,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    let description_text = &snippet_view.snippet.description;
    ctx.print_color(x, y, GREEN, BLACK, "Description:");
    ctx.print_color(x + x_offset, y, WHITE, BLACK, description_text);

    Ok(())
}

fn render_source_status(
    snippet_view: &SnippetView,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    let source_text = snippet_source_to_label(&snippet_view);
    ctx.print_color(x, y, GREEN, BLACK, "     Source:");
    ctx.print_color(x + x_offset, y, GRAY40, BLACK, source_text);

    Ok(())
}

fn render_transcribed_status(
    snippet_view: &SnippetView,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    let transcribed_text = format!("{}", &snippet_view.transcribed);
    let transcribed_text_color = if snippet_view.transcribed {
        GRAY40
    } else {
        WHITE
    };

    ctx.print_color(x, y, GREEN, BLACK, "Transcribed:");
    ctx.print_color(
        x + x_offset,
        y,
        transcribed_text_color,
        BLACK,
        transcribed_text,
    );

    Ok(())
}

fn render_has_border_status(
    has_border: bool,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    let has_border = format!("{}", has_border);
    ctx.print_color(x, y, YELLOW, BLACK, " Has Border:");
    ctx.print_color(x + x_offset, y, WHITE, BLACK, has_border);

    Ok(())
}

fn render_colored_status(
    colored: bool,
    ctx: &mut BTerm,
    x: u32,
    y: u32,
) -> Result<(), Box<dyn Error>> {
    let x_offset: u32 = 13;

    let colored = format!("{}", colored);
    ctx.print_color(x, y, YELLOW, BLACK, "    Colored:");
    ctx.print_color(x + x_offset, y, WHITE, BLACK, colored);

    Ok(())
}

fn snippet_source_to_label(snippet_view: &SnippetView) -> String {
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
