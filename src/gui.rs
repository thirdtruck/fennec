use std::error::Error;
use std::fs;

use crate::prelude::*;

pub fn map_key_to_glyph_segment(key: VirtualKeyCode) -> Option<Segment> {
    match key {
        VirtualKeyCode::W => Some(0),
        VirtualKeyCode::E => Some(1),
        VirtualKeyCode::R => Some(2),

        VirtualKeyCode::A => Some(3),
        VirtualKeyCode::S => Some(4),
        VirtualKeyCode::D => Some(5),
        VirtualKeyCode::F => Some(6),

        VirtualKeyCode::U => Some(7),
        VirtualKeyCode::I => Some(8),
        VirtualKeyCode::O => Some(9),
        VirtualKeyCode::P => Some(10),

        VirtualKeyCode::J => Some(11),
        VirtualKeyCode::K => Some(12),
        VirtualKeyCode::L => Some(13),
        VirtualKeyCode::Semicolon => Some(14),
        VirtualKeyCode::Q => Some(15),

        _ => None,
    }
}

pub fn on_modify_selected_glyph(_editor: &GlyphEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        if let Some(segment) = map_key_to_glyph_segment(key) {
            EditorEvent::ToggleSegmentOnSelectedGlyph(segment)
        } else {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,

                _ => EditorEvent::NoOp,
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_modify_glyph_set(_editor: &WordEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        if let Some(segment) = map_key_to_glyph_segment(key) {
            EditorEvent::ToggleSegmentOnSelectedGlyph(segment)
        } else {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,

                _ => EditorEvent::NoOp,
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_snippet_editor_input(editor: &SnippetEditor, ctx: &BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => EditorEvent::MoveWordCursorBackward,
            VirtualKeyCode::Down => EditorEvent::MoveWordCursorForward,
            VirtualKeyCode::Q => EditorEvent::ToggleGlyphEditingMode,
            _ => {
                let callbacks = WordEditorCallbacks {
                    on_modify_selected_glyph: Box::new(move |glyph_editor| {
                        on_modify_selected_glyph(glyph_editor, Some(key))
                    }),
                    on_modify_glyph_set: Box::new(move |word_editor| {
                        on_modify_glyph_set(word_editor, Some(key))
                    }),
                };

                editor.on_word_editor_input(callbacks)
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_notebook_editor_input(editor: &NotebookEditor, ctx: &BTerm) -> EditorEvent {
    let ctx = ctx.clone();
    let callback_ctx = ctx.clone();

    let callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent> =
        Box::new(move |snippet_editor| on_snippet_editor_input(snippet_editor, &callback_ctx));

    match editor.state() {
        NotebookEditorState::SelectingSnippet => {
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Return => EditorEvent::EnableSnippetEditingMode,
                    VirtualKeyCode::Up => EditorEvent::MoveSnippetCursorBackward,
                    VirtualKeyCode::Down => EditorEvent::MoveSnippetCursorForward,
                    VirtualKeyCode::Plus => EditorEvent::AddNewSnippetAtCursor,
                    _ => EditorEvent::NoOp,
                }
            } else {
                EditorEvent::NoOp
            }
        }
        NotebookEditorState::EditingSnippet => {
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Escape => EditorEvent::EnableSnippetNavigationMode,
                    _ => editor.on_snippet_editor_input(callback),
                }
            } else {
                editor.on_snippet_editor_input(callback)
            }
        }
    }
}

pub fn on_file_editor_input(editor: &FileEditor, ctx: &BTerm) -> EditorEvent {
    let ctx = ctx.clone();

    match editor.state() {
        FileEditorState::LoadRequestConfirmed => on_attempt_to_load_file(editor, &ctx),
        FileEditorState::SaveRequestConfirmed => on_attempt_to_save_file(editor, &ctx),
        _ => {
            if let Some(key) = ctx.key {
                match editor.state() {
                    FileEditorState::LoadRequestSucceeded => EditorEvent::ResetFileEditorToIdle,
                    FileEditorState::LoadRequestFailed(_) => EditorEvent::ResetFileEditorToIdle,
                    FileEditorState::SaveRequestSucceeded => EditorEvent::ResetFileEditorToIdle,
                    FileEditorState::SaveRequestFailed(_) => EditorEvent::ResetFileEditorToIdle,
                    FileEditorState::ConfirmingLoadRequest => match key {
                        VirtualKeyCode::Return => EditorEvent::ConfirmLoadFromFileRequest,
                        VirtualKeyCode::Escape => EditorEvent::ResetFileEditorToIdle,
                        _ => EditorEvent::NoOp,
                    },
                    FileEditorState::ConfirmingSaveRequest => match key {
                        VirtualKeyCode::Return => EditorEvent::ConfirmSaveToFileRequest,
                        VirtualKeyCode::Escape => EditorEvent::ResetFileEditorToIdle,
                        _ => EditorEvent::NoOp,
                    },
                    FileEditorState::Idle => match key {
                        VirtualKeyCode::F2 => EditorEvent::RequestSaveToFile,
                        VirtualKeyCode::F3 => EditorEvent::RequestLoadFromFile,
                        _ => {
                            let callback: Box<dyn Fn(&NotebookEditor) -> EditorEvent> =
                                Box::new(move |notebook_editor| {
                                    on_notebook_editor_input(notebook_editor, &ctx)
                                });

                            editor.on_notebook_editor_input(callback)
                        }
                    },
                    _ => EditorEvent::NoOp,
                }
            } else {
                EditorEvent::NoOp
            }
        }
    }
}

fn notebook_from_yaml_file(target_file: &str) -> Result<(Notebook, String), Box<dyn Error>> {
    let yaml = fs::read_to_string(target_file)?;
    let notebook: Notebook = serde_yaml::from_str(&yaml)?;

    Ok((notebook, yaml))
}

fn notebook_to_yaml_file(notebook: &Notebook, target_file: &str) -> Result<String, Box<dyn Error>> {
    let yaml = serde_yaml::to_string(notebook)?;

    fs::write(target_file, &yaml)?;

    Ok(yaml)
}

pub fn on_attempt_to_load_file(editor: &FileEditor, _ctx: &BTerm) -> EditorEvent {
    let file = editor.target_file();

    // TODO: Replace these println calls with proper logging
    println!("Loading notebook from: {}", &file);

    match notebook_from_yaml_file(&file) {
        Ok((notebook, yaml)) => {
            println!("Saved notebook to file: {}", &file);
            println!("YAML output:");
            println!("{}", yaml);

            EditorEvent::ReportLoadedFromFile(notebook)
        }
        Err(error) => {
            println!("Unable to load notebook from file");

            let error: FileEditorError = if error.is::<serde_yaml::Error>() {
                FileEditorError::new(
                    "Unable to parse YAML for the notebook".into(),
                    FileEditorErrorType::ParsingError,
                    file.clone(),
                    Some(&error),
                )
            } else {
                FileEditorError::new(
                    "Unable to load notebook from file".into(),
                    FileEditorErrorType::FileReadError,
                    file.clone(),
                    Some(&error),
                )
            };

            println!("{:?}", &error);

            EditorEvent::ReportFailedToLoadFromFile(error)
        }
    }
}

pub fn on_attempt_to_save_file(editor: &FileEditor, _ctx: &BTerm) -> EditorEvent {
    let file = editor.target_file();

    // TODO: Replace these println calls with proper logging
    println!("Saving notebook to: {}", &file);

    let notebook = editor.to_source();

    match notebook_to_yaml_file(&notebook, &file) {
        Ok(yaml) => {
            println!("Saved notebook to file");
            println!("YAML output:");
            println!("{}", yaml);

            EditorEvent::ReportSavedToFile
        }
        Err(error) => {
            println!("Unable to save notebook to file");

            let error = FileEditorError::new(
                "Unable to save notebook to file".into(),
                FileEditorErrorType::FileWriteError,
                file.clone(),
                Some(&error),
            );

            println!("{:?}", &error);

            EditorEvent::ReportFailedToSaveToFile(error)
        }
    }
}
