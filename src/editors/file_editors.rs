use std::error::Error;
use std::fmt;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileEditorState {
    Idle,
    ConfirmingSaveRequest,
    ConfirmingLoadRequest,
    SaveRequestConfirmed,
    LoadRequestConfirmed,
    SaveRequestSucceeded,
    LoadRequestSucceeded,
    SaveRequestFailed(FileEditorError),
    LoadRequestFailed(FileEditorError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileEditorErrorType {
    FileReadError,
    FileWriteError,
    ParsingError,
    #[allow(dead_code)]
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileEditorError {
    description: String,
    error_type: FileEditorErrorType,
    filename: String,
    inner_error_description: Option<String>,
}

impl FileEditorError {
    pub fn new(
        description: String,
        error_type: FileEditorErrorType,
        filename: String,
        inner_error: Option<&Box<dyn Error>>,
    ) -> Self {
        let inner_error_description = if let Some(inner_error) = inner_error {
            Some(inner_error.to_string())
        } else {
            None
        };

        Self {
            description,
            error_type,
            filename,
            inner_error_description,
        }
    }
}

impl fmt::Display for FileEditorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FileEditorError({:?}): {}\nFile: {:?}\nInner error: {:?}",
            &self.error_type, &self.description, &self.filename, &self.inner_error_description,
        )
    }
}

impl Error for FileEditorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Debug)]
pub struct FileEditor {
    notebook_editor: NotebookEditor,
    state: FileEditorState,
    target_file: String,
}

impl FileEditor {
    pub fn new(notebook: Notebook, filename: &str) -> Self {
        Self {
            notebook_editor: NotebookEditor::new(notebook).with_snippet_selected(0),
            target_file: filename.into(),
            state: FileEditorState::Idle,
        }
    }

    pub fn with_notebook(self, notebook: Notebook) -> Self {
        let notebook_editor = NotebookEditor::new(notebook).with_snippet_selected(0);

        Self {
            notebook_editor,
            ..self
        }
    }

    pub fn state(&self) -> FileEditorState {
        self.state.clone()
    }

    pub fn target_file(&self) -> String {
        self.target_file.clone()
    }

    pub fn with_state(self, state: FileEditorState) -> Self {
        Self { state, ..self }
    }

    pub fn on_input(&self, callback: Box<dyn Fn(&Self) -> EditorEvent>) -> EditorEvent {
        callback(self)
    }

    pub fn on_notebook_editor_input(
        &self,
        callback: Box<dyn Fn(&NotebookEditor) -> EditorEvent>,
    ) -> EditorEvent {
        self.notebook_editor.on_input(callback)
    }

    pub fn render_with<R>(&self, mut renderer: R) -> Result<(), Box<dyn Error>>
    where
        R: FnMut(FileEditorView) -> Result<(), Box<dyn Error>>,
    {
        renderer(self.to_view())
    }

    pub fn to_view(&self) -> FileEditorView {
        FileEditorView {
            notebook_view: self.notebook_editor.to_view(),
            state: self.state.clone(),
            target_file: self.target_file.clone(),
        }
    }

    pub fn to_source(&self) -> Notebook {
        self.notebook_editor.to_source()
    }
}

impl AppliesEditorEvents for FileEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::RequestLoadFromFile => {
                self.with_state(FileEditorState::ConfirmingLoadRequest)
            }
            EditorEvent::RequestSaveToFile => {
                self.with_state(FileEditorState::ConfirmingSaveRequest)
            }
            EditorEvent::ConfirmLoadFromFileRequest => {
                self.with_state(FileEditorState::LoadRequestConfirmed)
            }
            EditorEvent::ConfirmSaveToFileRequest => {
                self.with_state(FileEditorState::SaveRequestConfirmed)
            }
            EditorEvent::ReportLoadedFromFile(notebook) => self
                .with_notebook(notebook)
                .with_state(FileEditorState::LoadRequestSucceeded),
            EditorEvent::ReportSavedToFile => {
                self.with_state(FileEditorState::SaveRequestSucceeded)
            }
            EditorEvent::ReportFailedToLoadFromFile(error) => {
                self.with_state(FileEditorState::LoadRequestFailed(error))
            }
            EditorEvent::ReportFailedToSaveToFile(error) => {
                self.with_state(FileEditorState::SaveRequestFailed(error))
            }
            EditorEvent::ResetFileEditorToIdle => self.with_state(FileEditorState::Idle),
            _ => {
                let notebook_editor = self.notebook_editor.apply(event);

                Self {
                    notebook_editor,
                    ..self
                }
            }
        }
    }
}
