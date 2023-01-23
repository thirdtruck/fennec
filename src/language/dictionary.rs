use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DictionaryWord {
    glyphs: Vec<Glyph>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Definition {
    Undefined,
    Tentative(String),
    Confirmed(String),
}

impl From<&TunicWord> for DictionaryWord {
    fn from(tunic_word: &TunicWord) -> Self {
        Self {
            glyphs: tunic_word.glyphs()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    definition: Definition,
    notes: Vec<Note>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            definition: Definition::Undefined,
            notes: vec![],
        }
    }
}

impl Entry {
    pub fn new(definition: Definition, notes: Vec<Note>) -> Self {
        Self { definition, notes }
    }

    pub fn definition(&self) -> &Definition {
        &self.definition
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Dictionary {
    entries: HashMap<DictionaryWord, Entry>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn with_new_definition(self, tunic_word: &TunicWord, definition: String) -> Self {
        let definition = Definition::Tentative(definition);
        let entry = Entry::new(definition, vec![]);

        let mut entries = self.entries.clone();
        entries.insert(tunic_word.into(), entry);

        Self { entries, ..self }
    }

    pub fn with_annotation(self, tunic_word: &TunicWord, note: Note) -> Self {
        let word: DictionaryWord = tunic_word.into();
        let entry = if let Some(entry) = self.entries.get(&word) {
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

    pub fn get(&self, tunic_word: &TunicWord) -> Option<&Entry> {
        let word: DictionaryWord = tunic_word.into();

        self.entries.get(&word)
    }
}
