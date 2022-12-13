use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub snippets: Vec<Snippet>,
}

impl From<Vec<Snippet>> for Notebook {
    fn from(items: Vec<Snippet>) -> Self {
        Self {
            snippets: items,
        }
    }
}
