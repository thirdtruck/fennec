use serde::{Deserialize, Serialize};
use std::fmt;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TunicWord {
    glyphs: Vec<Glyph>,
    has_border: bool,
    colored: bool,
}

impl TunicWord {
    pub fn new(glyphs: Vec<Glyph>) -> Self {
        Self {
            glyphs,
            has_border: false,
            colored: false,
        }
    }

    pub fn with_colored_as(self, colored: bool) -> Self {
        Self {
            colored,
            ..self
        }
    }

    pub fn with_border_as(self, has_border: bool) -> Self {
        Self {
            has_border,
            ..self
        }
    }

    pub fn with_border_toggled(self) -> Self {
        Self {
            has_border: !self.has_border,
            ..self
        }
    }

    pub fn with_glyphs(self, glyphs: Vec<Glyph>) -> Self {
        Self {
            glyphs,
            ..self
        }
    }

    pub fn glyphs(&self) -> Vec<Glyph> {
        self.glyphs.clone()
    }

    pub fn has_border(&self) -> bool {
        self.has_border
    }

    pub fn colored(&self) -> bool {
        self.colored
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct EnglishWord {
    text: String,
}

impl EnglishWord {
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WordType {
    Tunic(TunicWord),
    English(EnglishWord),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Word {
    pub word_type: WordType,
}

impl Word {
    pub fn is_blank(&self) -> bool {
        match &self.word_type {
            WordType::Tunic(TunicWord { glyphs, .. }) => {
                if glyphs.len() == 1 {
                    glyphs.get(0).map_or(false, |g| g.is_blank())
                } else {
                    glyphs.is_empty()
                }
            }
            WordType::English(EnglishWord { text }) => text.len() == 0,
        }
    }

    pub fn apply_to_tunic_word(&self, callback: Box<dyn Fn(&TunicWord)>) {
        match &self.word_type {
            WordType::Tunic(word) => callback(&word),
            _ => ()
        };
    }

    pub fn apply_to_english_word(&self, callback: Box<dyn Fn(&EnglishWord)>) {
        match &self.word_type {
            WordType::English(word) => callback(&word),
            _ => ()
        };
    }
}

impl Default for Word {
    fn default() -> Self {
        let word_type = WordType::Tunic(TunicWord {
            glyphs: vec![],
            has_border: false,
            colored: false,
        });

        Self { word_type }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.word_type {
            WordType::Tunic(TunicWord { glyphs, .. }) => {
                let word = glyphs
                    .iter()
                    .map(|glyph| glyph.0.to_string())
                    .reduce(|word, glyph_value| word + ", " + &glyph_value)
                    .map_or("(Empty Tunic Word)".into(), |word| {
                        format!("Word::Tunic {}", word)
                    });

                write!(f, "{}", word)
            }
            WordType::English(EnglishWord { text }) => write!(f, "{}", text),
        }
    }
}

impl From<TunicWord> for Word {
    fn from(tunic_word: TunicWord) -> Self {
        Word {
            word_type: WordType::Tunic(tunic_word),
        }
    }
}

impl From<EnglishWord> for Word {
    fn from(english_word: EnglishWord) -> Self {
        Word {
            word_type: WordType::English(english_word),
        }
    }
}

impl From<String> for EnglishWord {
    fn from(text: String) -> Self {
        EnglishWord { text }
    }
}

impl From<&str> for EnglishWord {
    fn from(text: &str) -> Self {
        EnglishWord { text: text.to_string() }
    }
}

impl From<Vec<u16>> for Word {
    fn from(items: Vec<u16>) -> Self {
        let glyphs: Vec<Glyph> = items.iter().map(|c| Glyph(*c)).collect();

        let word_type = WordType::Tunic(TunicWord {
            glyphs,
            has_border: false,
            colored: false,
        });

        Self { word_type }
    }
}

impl From<&[u16]> for Word {
    fn from(items: &[u16]) -> Self {
        let glyphs: Vec<Glyph> = items.iter().map(|c| Glyph(*c)).collect();

        let word_type = WordType::Tunic(TunicWord {
            glyphs,
            has_border: false,
            colored: false,
        });

        Self { word_type }
    }
}

impl From<Vec<Glyph>> for Word {
    fn from(glyphs: Vec<Glyph>) -> Self {
        let word_type = WordType::Tunic(TunicWord {
            glyphs,
            has_border: false,
            colored: false,
        });

        Self { word_type }
    }
}

impl From<Glyph> for Word {
    fn from(glyph: Glyph) -> Self {
        let word_type = WordType::Tunic(TunicWord {
            glyphs: vec![glyph],
            has_border: false,
            colored: false,
        });

        Self { word_type }
    }
}

impl From<&str> for Word {
    fn from(text: &str) -> Self {
        Self { word_type: WordType::English(EnglishWord { text: text.to_string() }) }
    }
}

impl From<String> for Word {
    fn from(text: String) -> Self {
        Self { word_type: WordType::English(EnglishWord { text }) }
    }
}
