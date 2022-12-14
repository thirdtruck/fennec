use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    pub words: Vec<Word>,
    pub source: Option<Source>,
}

impl From<Vec<Word>> for Snippet {
    fn from(words: Vec<Word>) -> Self {
        Self {
            words,
            source: None,
        }
    }
}
