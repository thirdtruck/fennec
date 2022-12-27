use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashSet;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum NotebookEditorFilter {
    DraftSnippetsOnly,
}

impl NotebookEditorFilter {
    fn retains(&self, snippet: &Snippet) -> bool {
        match self {
            NotebookEditorFilter::DraftSnippetsOnly => !snippet.transcribed,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
    filters: HashSet<NotebookEditorFilter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SnippetFiltrationOutcome {
    snippet: Snippet,
    absolute_index: usize,
    relative_index: Option<usize>,
    retained: bool,
}

impl NotebookEditor {
    pub fn new(notebook: Notebook) -> Self {
        let mut filters = HashSet::new();
        filters.insert(NotebookEditorFilter::DraftSnippetsOnly);

        Self {
            selected_notebook: notebook,
            snippet_editor: None,
            selected_snippet_index: None,
            state: NotebookEditorState::EditingSnippet,
            filters,
        }
    }

    pub fn state(&self) -> NotebookEditorState {
        self.state.clone()
    }

    pub fn with_state(self, state: NotebookEditorState) -> Self {
        Self {
            state,
            ..self.clone()
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

    fn retained_snippet_outcomes(&self) -> Vec<SnippetFiltrationOutcome> {
        let snippets = self.selected_notebook.snippets.clone();
        let mut relative_index = 0;

        snippets
            .iter()
            .enumerate()
            .map(|(absolute_index, snippet)| {
                let retained = self.filters
                    .iter()
                    .fold(true, |passing, filter| passing && filter.retains(snippet));

                let current_relative_index = if retained { Some(relative_index) } else { None };

                if retained {
                    relative_index += 1
                }

                SnippetFiltrationOutcome {
                    snippet: snippet.clone(),
                    absolute_index,
                    relative_index: current_relative_index,
                    retained,
                }
            })
            .collect()
    }

    fn absolute_index_from_relative_move<M>(
        selected_snippet_index: Option<usize>,
        outcomes: Vec<SnippetFiltrationOutcome>,
        mover: M,
    ) -> usize
    where
        M: Fn(usize, usize) -> usize,
    {
        let selected_snippet_index = selected_snippet_index.unwrap_or(0);

        if let Some(outcome) = outcomes.get(selected_snippet_index) {
            let SnippetFiltrationOutcome { relative_index, .. } = outcome;
            let relative_index = relative_index.unwrap_or(0);

            let indices: Vec<(usize, usize)> = outcomes
                .iter()
                .filter(|oc| oc.retained && oc.relative_index.is_some())
                .map(|oc| (oc.absolute_index, oc.relative_index.unwrap_or(0)))
                .collect();

            let new_relative_index = mover(indices.len(), relative_index);

            indices
                .get(new_relative_index)
                .map_or(0, |(abs_index, _)| *abs_index)
        } else {
            0
        }
    }

    pub fn with_snippet_selection_moved_forward(self, amount: usize) -> Self {
        let new_index = Self::absolute_index_from_relative_move(
            self.selected_snippet_index,
            self.retained_snippet_outcomes(),
            |relative_count, relative_index| cmp::min(relative_count - 1, relative_index + amount),
        );

        self.with_snippet_selected(new_index)
    }

    pub fn with_snippet_selection_moved_backward(self, amount: usize) -> Self {
        let new_index = Self::absolute_index_from_relative_move(
            self.selected_snippet_index,
            self.retained_snippet_outcomes(),
            |_, relative_index| {
                if relative_index >= amount {
                    relative_index - amount
                } else {
                    0
                }
            },
        );

        self.with_snippet_selected(new_index)
    }

    pub fn with_new_snippet_at_cursor(self) -> Self {
        let mut snippets = self.selected_notebook.snippets.clone();
        let new_snippet: Snippet = Snippet::starting_snippet();

        let new_index = if let Some(selected_snippet_index) = self.selected_snippet_index {
            if selected_snippet_index + 1 == snippets.len() {
                snippets.push(new_snippet);
            } else {
                snippets.insert(selected_snippet_index + 1, new_snippet);
            }

            selected_snippet_index + 1
        } else {
            snippets.push(new_snippet);

            snippets.len() - 1
        };

        let selected_notebook = Notebook {
            snippets,
            ..self.selected_notebook
        };

        Self {
            selected_notebook,
            ..self
        }
        .with_snippet_selected(new_index)
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
            .retained_snippet_outcomes()
            .iter()
            .enumerate()
            .map(|(snippet_index, outcome)| {
                let snippet = &outcome.snippet;

                let selected = if let Some(selected_snippet_index) = self.selected_snippet_index {
                    snippet_index == selected_snippet_index
                } else {
                    false
                };

                if selected {
                    match self.snippet_editor.as_ref() {
                        Some(editor) => editor.to_view(true, outcome.retained),
                        None => {
                            dbg!("Missing SnippetEditor");
                            dbg!(&self.snippet_editor);

                            SnippetEditor::new(snippet.clone()).to_view(false, outcome.retained)
                        }
                    }
                } else {
                    SnippetEditor::new(snippet.clone()).to_view(false, outcome.retained)
                }
            })
            .collect();

        NotebookView {
            state: self.state.clone(),
            snippet_views,
        }
    }

    pub fn to_source(&self) -> Notebook {
        self.selected_notebook.clone()
    }
}

impl AppliesEditorEvents for NotebookEditor {
    fn apply(self, event: EditorEvent) -> Self {
        match &event {
            EditorEvent::EnableSnippetEditingMode => {
                self.with_state(NotebookEditorState::EditingSnippet)
            }
            EditorEvent::EnableSnippetNavigationMode => {
                self.with_state(NotebookEditorState::SelectingSnippet)
            }
            EditorEvent::MoveSnippetCursorBackward => self.with_snippet_selection_moved_backward(1),
            EditorEvent::MoveSnippetCursorForward => self.with_snippet_selection_moved_forward(1),
            EditorEvent::AddNewSnippetAtCursor => self.with_new_snippet_at_cursor(),
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
