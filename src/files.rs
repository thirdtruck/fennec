use std::error::Error;
use std::fs;

use crate::prelude::*;

pub fn notebook_from_yaml_file(filename: &str) -> Result<Notebook, Box<dyn Error>> {
    let yaml = fs::read_to_string(filename)?;
    let notebook: Notebook = serde_yaml::from_str(&yaml)?;

    Ok(notebook)
}
