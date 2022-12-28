use std::error::Error;

use crate::prelude::*;

pub fn render_file_editor_view_onto(
    view: &FileEditorView,
    ctx: &mut BTerm,
) -> Result<(), Box<dyn Error>> {
    ctx.set_active_console(FILE_CONSOLE);
    ctx.cls();

    let x: i32 = 1;
    let y: i32 = (SCREEN_HEIGHT - 3).try_into()?;

    match &view.state {
        FileEditorState::LoadRequestSucceeded => {
            let text = format!("Loaded notebook from {}", view.target_file);
            ctx.print_color(x, y, GREEN, BLACK, text);
        }
        FileEditorState::LoadRequestFailed(error) => {
            let text = format!("Failed to load notebook from {}", view.target_file);
            ctx.print_color(x, y - 1, RED, BLACK, text);
            ctx.print_color(x, y, WHITE, BLACK, error.to_string());
        }
        FileEditorState::SaveRequestSucceeded => {
            let text = format!("Saved notebook to {}", view.target_file);
            ctx.print_color(x, y, GREEN, BLACK, text);
        }
        FileEditorState::SaveRequestFailed(error) => {
            let text = format!("Failed to save notebook to {}", view.target_file);
            ctx.print_color(x, y - 1, RED, BLACK, text);
            ctx.print_color(x, y, WHITE, BLACK, error.to_string());
        }
        FileEditorState::ConfirmingLoadRequest => {
            let text = format!(
                "Load the notebook from {}? Press Enter/Return to confirm or Escape to cancel",
                view.target_file
            );
            ctx.print_color(x, y, YELLOW, BLACK, text);
        }
        FileEditorState::ConfirmingSaveRequest => {
            let text = format!(
                "Save the notebook to {}? Press Enter/Return to confirm or Escape to cancel",
                view.target_file
            );
            ctx.print_color(x, y, YELLOW, BLACK, text);
        }
        FileEditorState::LoadRequestConfirmed => (),
        FileEditorState::SaveRequestConfirmed => (),
        FileEditorState::Idle => (),
    };

    Ok(())
}
