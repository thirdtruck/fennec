use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NotebookEditor {
    selected_notebook: Notebook,
    #[serde(skip_serializing)]
    snippet_editor: Option<SnippetEditor>,
    #[serde(skip_serializing)]
    selected_snippet_index: Option<usize>,
}

impl AppliesEditorEvents for NotebookEditor {
    fn apply(self, event: EditorEvent) -> Self {
        self
    }
}
