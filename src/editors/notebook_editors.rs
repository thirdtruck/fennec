use serde::{Deserialize, Serialize};
use std::cmp;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NotebookEditorState {
    EditingSnippet,
    SelectingSnippet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotebookEditor {
    selected_notebook: Notebook,
    snippet_editor: Option<SnippetEditor>,
    selected_snippet_index: Option<usize>,
    state: NotebookEditorState,
}

impl NotebookEditor {
    pub fn new(notebook: Notebook) -> Self {
        Self {
            selected_notebook: notebook,
            snippet_editor: None,
            selected_snippet_index: None,
            state: NotebookEditorState::EditingSnippet,
        }
    }

    pub fn state(&self) -> NotebookEditorState {
        self.state
    }

    pub fn with_state(self, state: NotebookEditorState) -> Self {
        Self {
            state,
            ..self
        }
    }

    pub fn with_snippet_selected(self, index: usize) -> Self {
        let snippets = self.selected_notebook.snippets.clone();

        if let Some(snippet) = snippets.get(index) {
            let editor = SnippetEditor::new(snippet.clone()).with_word_selected(0);

            Self {
                snippet_editor: Some(editor),
                selected_snippet_index: Some(index),
                ..self
            }
        } else {
            self
        }
    }

    pub fn with_snippet_selection_moved_forward(self, amount: usize) -> Self {
        let new_index = if let Some(index) = self.selected_snippet_index {
            cmp::min(self.selected_notebook.snippets.len(), index + amount)
        } else {
            0
        };

        self.with_snippet_selected(new_index)
    }

    pub fn with_snippet_selection_moved_backward(self, amount: usize) -> Self {
        let new_index = if let Some(index) = self.selected_snippet_index {
            if index >= amount {
                index - amount
            } else {
                0
            }
        } else {
            0
        };

        self.with_snippet_selected(new_index)
    }

    pub fn on_input(&self, callback: Box<dyn Fn(&Self) -> EditorEvent>) -> EditorEvent {
        callback(self)
    }

    pub fn on_snippet_editor_input(
        &self,
        callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent>,
    ) -> EditorEvent {
        if let Some(editor) = &self.snippet_editor {
            editor.on_input(callback)
        } else {
            EditorEvent::NoOp
        }
    }

    #[allow(dead_code)]
    pub fn render_with<R>(&self, mut renderer: R)
    where
        R: FnMut(NotebookView, usize),
    {
        renderer(self.to_view(), 0)
    }

    pub fn to_view(&self) -> NotebookView {
        let snippet_views: Vec<SnippetView> = self
            .selected_notebook
            .snippets
            .iter()
            .enumerate()
            .map(|(snippet_index, snippet)| {
                let selected = if let Some(selected_snippet_index) = self.selected_snippet_index {
                    snippet_index == selected_snippet_index
                } else {
                    false
                };

                if selected {
                    match self.snippet_editor.as_ref() {
                        Some(editor) => editor.to_view(true),
                        None => {
                            dbg!("Missing SnippetEditor");
                            dbg!(&self.snippet_editor);

                            SnippetEditor::new(snippet.clone()).to_view(false)
                        }
                    }
                } else {
                    SnippetEditor::new(snippet.clone()).to_view(false)
                }
            })
            .collect();

        NotebookView { state: self.state, snippet_views }
    }

    pub fn to_source(&self) -> Notebook {
        self.selected_notebook.clone()
    }
}

impl AppliesEditorEvents for NotebookEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match &event {
            EditorEvent::EnableSnippetEditingMode => self.with_state(NotebookEditorState::EditingSnippet),
            EditorEvent::EnableSnippetNavigationMode => self.with_state(NotebookEditorState::SelectingSnippet),
            EditorEvent::MoveSnippetCursorBackward => self.with_snippet_selection_moved_backward(1),
            EditorEvent::MoveSnippetCursorForward => self.with_snippet_selection_moved_forward(1),
            _ => {
                if let Some(editor) = self.snippet_editor {
                    let snippet_editor = editor.apply(event);

                    let mut notebook = self.selected_notebook.clone();

                    if let Some(index) = self.selected_snippet_index {
                        if let Some(snippet) = notebook.snippets.get_mut(index) {
                            *snippet = snippet_editor.selected_snippet();
                        }
                    }

                    Self {
                        selected_notebook: notebook,
                        snippet_editor: Some(snippet_editor),
                        ..self
                    }
                } else {
                    self
                }
            }
        }
    }
}
