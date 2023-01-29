use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct DictionaryWord {
    glyphs: Vec<Glyph>,
}

impl DictionaryWord {
    pub fn glyphs(&self) -> Vec<Glyph> {
        self.glyphs.clone()
    }
}

impl From<TunicWord> for DictionaryWord {
    fn from(tunic_word: TunicWord) -> Self {
        Self {
            glyphs: tunic_word.glyphs()
        }
    }
}

impl From<&TunicWord> for DictionaryWord {
    fn from(tunic_word: &TunicWord) -> Self {
        Self {
            glyphs: tunic_word.glyphs()
        }
    }
}

impl fmt::Display for DictionaryWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word = self
            .glyphs
            .iter()
            .map(|glyph| glyph.0.to_string())
            .reduce(|word, glyph_value| word + " " + &glyph_value)
            .map_or("(Empty Tunic Word)".into(), |word| {
                format!("TunicWord: {}", word)
            });

        write!(f, "{}", word)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Definition {
    Undefined,
    Tentative(String),
    Confirmed(String),
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = match self {
            Self::Undefined => "Undefined".to_owned(),
            Self::Tentative(text) => format!("Tenative({text})"),
            Self::Confirmed(text) => format!("Confirmed({text})"),
        };

        write!(f, "Definition({formatted})")
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

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let definition = self.definition.clone();
        let notes: String = self
            .notes
            .iter()
            .map(|note| format!("Note({})", note.0.clone()))
            .reduce(|combined_notes, note| combined_notes + ", " + &note)
            .unwrap_or("[]".into());

        write!(f, "Entry {{ Definition: {definition}, Notes: {notes} }}")
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

    pub fn with_new_complete_definition(self, word: &DictionaryWord, entry: &Entry) -> Self {
        let mut entries = self.entries.clone();
        entries.insert(word.clone(), entry.clone());

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

    pub fn get(&self, word: &DictionaryWord) -> Option<&Entry> {
        self.entries.get(word)
    }

    pub fn entries(&self) -> &HashMap<DictionaryWord, Entry> {
        &self.entries
    }
}
