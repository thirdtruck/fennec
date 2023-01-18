use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    definition: Option<String>,
    notes: Vec<Note>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            definition: None,
            notes: vec![],
        }
    }
}

impl Entry {
    pub fn new(definition: Option<String>, notes: Vec<Note>) -> Self {
        Self { definition, notes }
    }

    pub fn definition(&self) -> &Option<String> {
        &self.definition
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dictionary {
    entries: HashMap<Word, Entry>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn with_new_definition(self, word: &Word, definition: String) -> Self {
        let entry = Entry::new(Some(definition), vec![]);

        let mut entries = self.entries.clone();
        entries.insert(word.clone(), entry);

        Self { entries, ..self }
    }

    pub fn with_annotation(self, word: &Word, note: Note) -> Self {
        let entry = if let Some(entry) = self.entries.get(word) {
            let mut entry = entry.clone();
            entry.notes.push(note);
            entry
        } else {
            Entry::default()
        };

        let mut entries = self.entries.clone();
        entries.insert(word.clone(), entry);

        Self { entries, ..self }
    }

    pub fn get(&self, word: &Word) -> Option<&Entry> {
        self.entries.get(word)
    }
}
