use std::error::Error;
use std::fs;

use crate::prelude::*;
pub fn map_keys_to_glyph_segments(key: VirtualKeyCode, shift_key: bool) -> Vec<Segment> {
    match key {
        VirtualKeyCode::W if shift_key => vec![1, 3],
        VirtualKeyCode::R if shift_key => vec![1, 3],

        VirtualKeyCode::E => vec![2, 6],
        VirtualKeyCode::D => vec![2, 6],

        VirtualKeyCode::I => vec![10, 13],
        VirtualKeyCode::K => vec![10, 13],

        VirtualKeyCode::P => vec![4, 8],
        VirtualKeyCode::A => vec![4, 8],

        VirtualKeyCode::U if shift_key => vec![9, 11],
        VirtualKeyCode::O if shift_key => vec![9, 11],

        VirtualKeyCode::J if shift_key => vec![12, 14],
        VirtualKeyCode::L if shift_key => vec![12, 14],

        VirtualKeyCode::S if shift_key => vec![5, 7],
        VirtualKeyCode::F if shift_key => vec![5, 7],

        VirtualKeyCode::W => vec![1],
        VirtualKeyCode::R => vec![3],

        VirtualKeyCode::S => vec![5],
        VirtualKeyCode::F => vec![7],

        VirtualKeyCode::U => vec![9],
        VirtualKeyCode::O => vec![11],

        VirtualKeyCode::J => vec![12],
        VirtualKeyCode::L => vec![14],
        VirtualKeyCode::Semicolon => vec![15],

        _ => vec![],
    }
}

pub fn on_modify_selected_glyph(_editor: &GlyphEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        let segments = map_keys_to_glyph_segments(key, ctx.shift);

        if segments.is_empty() {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,
                VirtualKeyCode::Key8 => EditorEvent::ToggleWordIsColoredState,
                VirtualKeyCode::Key9 => EditorEvent::ToggleWordHasABorderState,

                _ => EditorEvent::NoOp,
            }
        } else {
            EditorEvent::ToggleSegmentsOnSelectedGlyph(segments)
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_modify_tunic_word(editor: &TunicWordEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::F4 => EditorEvent::PrintWord(editor.word().into()),
            _ => EditorEvent::NoOp,
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_modify_english_word(_editor: &EnglishWordEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Back => EditorEvent::DeleteWordAtCursor,
            _ => EditorEvent::NoOp,
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_snippet_editor_input(editor: &SnippetEditor, ctx: BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up if ctx.shift => EditorEvent::MoveWordsViewSliceBackward(1),
            VirtualKeyCode::Down if ctx.shift => EditorEvent::MoveWordsViewSliceForward(1),
            VirtualKeyCode::Up => EditorEvent::MoveWordCursorBackward,
            VirtualKeyCode::Down => EditorEvent::MoveWordCursorForward,
            VirtualKeyCode::Key0 => EditorEvent::ToggleSnippetTranscriptionState,
            VirtualKeyCode::Return if ctx.shift && ctx.control => EditorEvent::AddNewEnglishWordAtCursor("...".to_owned()),
            VirtualKeyCode::Return if ctx.shift => EditorEvent::AddNewEnglishWordAtCursor(".".to_owned()),
            VirtualKeyCode::Period => EditorEvent::AddNewEnglishWordAtCursor(".".to_owned()),
            VirtualKeyCode::Minus => EditorEvent::AddNewEnglishWordAtCursor("---".to_owned()),
            VirtualKeyCode::Comma => EditorEvent::AddNewEnglishWordAtCursor(",".to_owned()),
            VirtualKeyCode::Slash => EditorEvent::AddNewEnglishWordAtCursor("?".to_owned()),
            VirtualKeyCode::Key1 => EditorEvent::AddNewEnglishWordAtCursor("!".to_owned()),
            VirtualKeyCode::Z => EditorEvent::AddNewEnglishWordAtCursor("PLACEHOLDER".to_owned()),
            VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
            _ => {
                let glyph_ctx = ctx.clone();
                let tunic_word_ctx = ctx.clone();
                let english_word_ctx = ctx.clone();
                let callbacks = WordEditorCallbacks {
                    on_modify_selected_glyph: Box::new(move |glyph_editor| {
                        on_modify_selected_glyph(glyph_editor, glyph_ctx.clone())
                    }),
                    on_modify_tunic_word: Box::new(move |tunic_word_editor| {
                        on_modify_tunic_word(tunic_word_editor, tunic_word_ctx.clone())
                    }),
                    on_modify_english_word: Box::new(move |english_word_editor| {
                        on_modify_english_word(english_word_editor, english_word_ctx.clone())
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

    let callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent> = Box::new(move |snippet_editor| {
        on_snippet_editor_input(snippet_editor, callback_ctx.clone())
    });

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

pub fn dictionary_from_yaml_file(
    target_file: &str,
) -> Result<(Dictionary, String), Box<dyn Error>> {
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
