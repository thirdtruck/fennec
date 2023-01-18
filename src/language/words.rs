use serde::{Deserialize, Serialize};
use std::fmt;

use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Word {
    Tunic {
        glyphs: Vec<Glyph>,
        has_border: bool,
        colored: bool,
    },
    English(String),
}

impl Word {
    pub fn is_blank(&self) -> bool {
        match self {
            Word::Tunic { glyphs, .. } => {
                if glyphs.len() == 1 {
                    glyphs.get(0).map_or(false, |g| g.is_blank())
                } else {
                    glyphs.is_empty()
                }
            }
            Word::English(string) => string.len() == 0,
        }
    }

    pub fn has_border(&self) -> bool {
        match self {
            Word::Tunic { has_border, .. } => *has_border,
            Word::English(_) => false,
        }
    }

    pub fn glyphs(&self) -> Vec<Glyph> {
        match self {
            Word::Tunic { glyphs, .. } => glyphs.clone(),
            Word::English(_) => vec![],
        }
    }

    pub fn with_border_toggled(self) -> Self {
        match self {
            Self::Tunic { glyphs, has_border, colored } => Self::Tunic {
                has_border: !has_border,
                glyphs,
                colored,
            },
            Self::English(_) => self,
        }
    }

    pub fn with_glyphs(self, glyphs: Vec<Glyph>) -> Self {
        match self {
            Self::Tunic { has_border, colored, .. } => Self::Tunic {
                glyphs,
                has_border,
                colored,
            },
            Self::English(_) => self,
        }
    }
}

impl Default for Word {
    fn default() -> Self {
        Self::Tunic {
            glyphs: vec![],
            has_border: false,
            colored: false,
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Word::Tunic { glyphs, .. } => {
                let word = glyphs
                    .iter()
                    .map(|glyph| glyph.0.to_string())
                    .reduce(|word, glyph_value| word + ", " + &glyph_value)
                    .map_or("(Empty Tunic Word)".into(), |word| format!("Word::Tunic {}", word));

                write!(f, "{}", word)
            }
            Word::English(text) => write!(f, "{}", text),
        }
    }
}

impl From<Vec<u16>> for Word {
    fn from(items: Vec<u16>) -> Self {
        let glyphs: Vec<Glyph> = items.iter().map(|c| Glyph(*c)).collect();

        Self::Tunic {
            glyphs,
            has_border: false,
            colored: false,
        }
    }
}

impl From<&[u16]> for Word {
    fn from(items: &[u16]) -> Self {
        let glyphs: Vec<Glyph> = items.iter().map(|c| Glyph(*c)).collect();

        Self::Tunic {
            glyphs,
            has_border: false,
            colored: false,
        }
    }
}

impl From<Vec<Glyph>> for Word {
    fn from(glyphs: Vec<Glyph>) -> Self {
        Self::Tunic {
            glyphs,
            has_border: false,
            colored: false,
        }
    }
}

impl From<Glyph> for Word {
    fn from(glyph: Glyph) -> Self {
        Self::Tunic {
            glyphs: vec![glyph],
            has_border: false,
            colored: false,
        }
    }
}

impl From<&str> for Word {
    fn from(string: &str) -> Self {
        Self::English(string.to_string())
    }
}

impl From<String> for Word {
    fn from(string: String) -> Self {
        Self::English(string)
    }
}
