use std::fs;

use fennec::prelude::DEFAULT_NOTEBOOK_FILE;

mod old_version {
    use serde::{Deserialize, Serialize};

    use fennec::prelude::{Glyph, Source, Note};

    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub enum Word {
        Tunic(Vec<Glyph>),
        English(String),
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Snippet {
        pub words: Vec<Word>,
        pub source: Option<Source>,
        pub description: String,
        pub notes: Vec<Note>,
        pub transcribed: bool,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Notebook {
        pub snippets: Vec<Snippet>,
    }
}

impl old_version::Word {
    pub fn migrated(&self) -> fennec::prelude::Word {
        match self {
            Self::Tunic(glyphs) => fennec::prelude::Word::Tunic {
                glyphs: glyphs.clone(),
                has_border: false,
                colored: false,
            },
            Self::English(text) => fennec::prelude::Word::English(text.clone()),
        }
    }
}

impl old_version::Snippet {
    pub fn migrated(&self) -> fennec::prelude::Snippet {
        let words: Vec<fennec::prelude::Word> = self.words
            .iter()
            .map(|word| word.migrated())
            .collect();

        fennec::prelude::Snippet {
            words,
            source: self.source.clone(),
            description: self.description.clone(),
            notes: self.notes.clone(),
            transcribed: self.transcribed.clone(),
        }
    }
}

impl From<old_version::Notebook> for fennec::prelude::Notebook {
    fn from(old_notebook: old_version::Notebook) -> Self {
        let snippets: Vec<fennec::prelude::Snippet> = old_notebook.snippets
            .iter()
            .map(|snip| snip.migrated())
            .collect();

        Self {
            snippets,
            version: 2,
        }
    }
}

fn main() {
    let old_yaml = fs::read_to_string(DEFAULT_NOTEBOOK_FILE).unwrap();
    let old_notebook: old_version::Notebook = serde_yaml::from_str(&old_yaml).unwrap();

    let new_notebook: fennec::prelude::Notebook = old_notebook.into();
    let new_yaml = serde_yaml::to_string(&new_notebook).unwrap();

    //fs::write(target_file, &yaml)?;
    print!("{}", new_yaml);
}
