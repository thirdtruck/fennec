use std::error::Error;
use std::fs;

use crate::prelude::*;
pub fn map_keys_to_glyph_segments(key: VirtualKeyCode, shift_key: bool) -> Vec<Segment> {
    match key {
        VirtualKeyCode::W if shift_key => vec![1, 3],
        VirtualKeyCode::R if shift_key => vec![1, 3],

        VirtualKeyCode::E if shift_key => vec![2, 6],
        VirtualKeyCode::D if shift_key => vec![2, 6],

        VirtualKeyCode::I if shift_key => vec![10, 13],
        VirtualKeyCode::K if shift_key => vec![10, 13],

        VirtualKeyCode::P if shift_key => vec![4, 8],
        VirtualKeyCode::A if shift_key => vec![4, 8],

        VirtualKeyCode::U if shift_key => vec![9, 11],
        VirtualKeyCode::O if shift_key => vec![9, 11],

        VirtualKeyCode::J if shift_key => vec![12, 14],
        VirtualKeyCode::L if shift_key => vec![12, 14],

        VirtualKeyCode::S if shift_key => vec![5, 7],
        VirtualKeyCode::F if shift_key => vec![5, 7],

        VirtualKeyCode::W => vec![1],
        VirtualKeyCode::E => vec![2],
        VirtualKeyCode::R => vec![3],

        VirtualKeyCode::A => vec![4],
        VirtualKeyCode::S => vec![5],
        VirtualKeyCode::D => vec![6],
        VirtualKeyCode::F => vec![7],

        VirtualKeyCode::U => vec![9],
        VirtualKeyCode::I => vec![10],
        VirtualKeyCode::O => vec![11],
        VirtualKeyCode::P => vec![8],

        VirtualKeyCode::J => vec![12],
        VirtualKeyCode::K => vec![13],
        VirtualKeyCode::L => vec![14],
        VirtualKeyCode::Semicolon => vec![15],

        _ => vec![]
    }
}

pub fn on_modify_selected_glyph(_editor: &GlyphEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        let segments = map_keys_to_glyph_segments(key, ctx.shift);

        if segments.is_empty() {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,

                _ => EditorEvent::NoOp,
            }
        } else {
            EditorEvent::ToggleSegmentsOnSelectedGlyph(segments)
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_modify_glyph_set(_editor: &WordEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        let segments = map_keys_to_glyph_segments(key, ctx.shift);

        if segments.is_empty() {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,

                _ => EditorEvent::NoOp,
            }
        } else {
            EditorEvent::ToggleSegmentsOnSelectedGlyph(segments)
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_snippet_editor_input(editor: &SnippetEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => EditorEvent::MoveWordCursorBackward,
            VirtualKeyCode::Down => EditorEvent::MoveWordCursorForward,
            VirtualKeyCode::Q => EditorEvent::ToggleGlyphEditingMode,
            VirtualKeyCode::Key0 => EditorEvent::ToggleSnippetTranscriptionState,
            _ => {
                let glyph_ctx = ctx.clone();
                let word_ctx = ctx.clone();
                let callbacks = WordEditorCallbacks {
                    on_modify_selected_glyph: Box::new(move |glyph_editor| {
                        on_modify_selected_glyph(glyph_editor, glyph_ctx.clone())
                    }),
                    on_modify_glyph_set: Box::new(move |word_editor| {
                        on_modify_glyph_set(word_editor, word_ctx.clone())
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
        Box::new(move |snippet_editor| on_snippet_editor_input(snippet_editor, callback_ctx.clone()));

    match editor.state() {
        NotebookEditorState::SelectingSnippet => {
            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Escape => EditorEvent::EnableSnippetEditingMode,
                    VirtualKeyCode::Return => EditorEvent::EnableSnippetEditingMode,
                    VirtualKeyCode::Up => EditorEvent::MoveSnippetCursorBackward,
                    VirtualKeyCode::K => EditorEvent::MoveSnippetCursorBackward,
                    VirtualKeyCode::Down => EditorEvent::MoveSnippetCursorForward,
                    VirtualKeyCode::J => EditorEvent::MoveSnippetCursorForward,
                    VirtualKeyCode::Plus => EditorEvent::AddNewSnippetAtCursor,
                    VirtualKeyCode::T => EditorEvent::ToggleHasBeenTranscribedFilter,
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

pub fn notebook_from_yaml_file(target_file: &str) -> Result<(Notebook, String), Box<dyn Error>> {
    let yaml = fs::read_to_string(target_file)?;
    let notebook: Notebook = serde_yaml::from_str(&yaml)?;

    Ok((notebook, yaml))
}

pub fn notebook_to_yaml_file(
    notebook: &Notebook,
    target_file: &str,
) -> Result<String, Box<dyn Error>> {
    let yaml = serde_yaml::to_string(notebook)?;

    fs::write(target_file, &yaml)?;

    Ok(yaml)
}

pub fn dictionary_from_yaml_file(target_file: &str) -> Result<(Dictionary, String), Box<dyn Error>> {
    let yaml = fs::read_to_string(target_file)?;
    let dictionary: Dictionary = serde_yaml::from_str(&yaml)?;

    Ok((dictionary, yaml))
}

pub fn dictionary_to_yaml_file(
    dictionary: &Dictionary,
    target_file: &str,
) -> Result<String, Box<dyn Error>> {
    let yaml = serde_yaml::to_string(dictionary)?;

    fs::write(target_file, &yaml)?;

    Ok(yaml)
}

pub fn on_attempt_to_load_file(editor: &FileEditor, _ctx: &BTerm) -> EditorEvent {
    let file = editor.target_file();

    // TODO: Replace these println calls with proper logging
    println!("Loading notebook from: {}", &file);

    match notebook_from_yaml_file(&file) {
        Ok((notebook, _yaml)) => {
            println!("Saved notebook to file: {}", &file);

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
        Ok(_yaml) => {
            println!("Saved notebook to file");

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
