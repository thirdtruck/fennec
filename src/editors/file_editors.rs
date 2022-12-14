use std::error::Error;
use std::fmt;
use std::fs;

use crate::prelude::*;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum FileEditorState {
    Idle,
    ConfirmingSaveRequest,
    ConfirmingLoadRequest,
    SaveRequestSucceeded,
    LoadRequestSucceeded,
    SaveRequestFailed(FileEditorError),
    LoadRequestFailed(FileEditorError),
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileEditorErrorType {
    FileReadError,
    FileWriteError,
    ParsingError,
    Other,
}

#[derive(Clone, Debug)]
pub struct FileEditorError {
    description: String,
    error_type: FileEditorErrorType,
    filename: String,
    inner_error_description: Option<String>,
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

    pub fn on_input(&self, callback: Box<dyn Fn(&Self) -> EditorEvent>) -> EditorEvent {
        callback(self)
    }

    pub fn on_notebook_editor_input(
        &self,
        callback: Box<dyn Fn(&NotebookEditor) -> EditorEvent>,
    ) -> EditorEvent {
        self.notebook_editor.on_input(callback)
    }

    pub fn render_with<R>(&self, mut renderer: R)
    where
        R: FnMut(FileView),
    {
        renderer(self.to_view())
    }

    pub fn with_file_load_attempted(self) -> Self {
        println!("Loading notebook from: {}", &self.target_file);

        match &self.notebook_from_yaml_file() {
            Ok((notebook, yaml)) => {
                println!("Saved notebook to file: {}", &self.target_file);
                println!("YAML output:");
                println!("{}", yaml);

                Self {
                    state: FileEditorState::LoadRequestSucceeded,
                    notebook_editor: NotebookEditor::new(notebook.clone()).with_snippet_selected(0),
                    ..self
                }
            }
            Err(error) => {
                println!("Unable to load notebook from file");

                let parse_error = error.downcast_ref::<serde_yaml::Error>();

                let error: FileEditorError = if let Some(parse_error) = parse_error {
                    FileEditorError {
                        description: "Unable to parse YAML for the notebook".into(),
                        error_type: FileEditorErrorType::ParsingError,
                        filename: self.target_file.clone(),
                        inner_error_description: Some(parse_error.to_string()),
                    }
                } else {
                    FileEditorError {
                        description: "Unable to save notebook to file".into(),
                        error_type: FileEditorErrorType::FileWriteError,
                        filename: self.target_file.clone(),
                        inner_error_description: Some(error.to_string()),
                    }
                };

                println!("{:?}", error);

                Self {
                    state: FileEditorState::LoadRequestFailed(error),
                    ..self
                }
            }
        }
    }

    pub fn with_file_save_attempted(self) -> Self {
        // TODO: Move these printlns out of the editor and into the GUI layer
        println!("Saving notebook to: {}", &self.target_file);

        match &self.notebook_to_yaml_file() {
            Ok(yaml) => {
                println!("Saved notebook to file");
                println!("YAML output:");
                println!("{}", yaml);

                Self {
                    state: FileEditorState::SaveRequestSucceeded,
                    ..self
                }
            }
            Err(error) => {
                println!("Unable to save notebook to file");

                let error = FileEditorError {
                    description: "Unable to save notebook to file".into(),
                    error_type: FileEditorErrorType::FileWriteError,
                    filename: self.target_file.clone(),
                    inner_error_description: Some(error.to_string()),
                };

                println!("{:?}", error);

                Self {
                    state: FileEditorState::SaveRequestFailed(error),
                    ..self
                }
            }
        }
    }

    pub fn to_view(&self) -> FileView {
        FileView {
            notebook_view: self.notebook_editor.to_view(),
            state: self.state.clone(),
            target_file: self.target_file.clone(),
        }
    }

    fn notebook_to_yaml_file(&self) -> Result<String, Box<dyn Error>> {
        let yaml = serde_yaml::to_string(&self.notebook_editor.to_source())?;

        fs::write(&self.target_file, &yaml)?;

        Ok(yaml)
    }

    fn notebook_from_yaml_file(&self) -> Result<(Notebook, String), Box<dyn Error>> {
        let yaml = fs::read_to_string(&self.target_file)?;
        let notebook: Notebook = serde_yaml::from_str(&yaml)?;

        Ok((notebook, yaml))
    }
}

impl AppliesEditorEvents for FileEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match event {
            EditorEvent::RequestSaveToFile => self.with_file_save_attempted(),
            EditorEvent::RequestLoadFromFile => self.with_file_load_attempted(),
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
