use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub const VERSION: usize = 2;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub version: usize,
    pub snippets: Vec<Snippet>,
}

impl From<Vec<Snippet>> for Notebook {
    fn from(items: Vec<Snippet>) -> Self {
        Self { snippets: items, version: VERSION }
    }
}
