use std::error::Error;
use std::fs;

use crate::prelude::*;

pub fn notebook_from_yaml_file(filename: &str) -> Result<Notebook, Box<dyn Error>> {
    let yaml = fs::read_to_string(filename)?;
    let notebook: Notebook = serde_yaml::from_str(&yaml)?;

    Ok(notebook)
}

pub fn notebook_to_yaml_file(
    notebook: &Notebook,
    filename: &str,
) -> Result<String, Box<dyn Error>> {
    match serde_yaml::to_string(&notebook) {
        Ok(yaml) => {
            fs::write(filename, &yaml)?;
            Ok(yaml)
        }
        Err(error) => Err(Box::new(error)),
    }
}
