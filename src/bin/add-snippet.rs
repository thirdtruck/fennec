use fennec::prelude::*;

fn main() {
    println!("Loading notebook...");
    match notebook_from_yaml_file(DEFAULT_NOTEBOOK_FILE) {
        Ok((mut notebook, _yaml)) => {
            println!("Adding snippet...");

            let english_word: Word = "SomeEnglish".into();
            let tunic_word: Word = vec![0x99].into();

            let snippet = Snippet {
                words: vec![english_word, tunic_word].into(),
                source: Some(Source::ManualPageNumber(0)),
                description: "ADD_DESCRIPTION_HERE".into(),
            };
                
            notebook.snippets.push(snippet);

            println!("Saving notebook...");

            match notebook_to_yaml_file(&notebook, DEFAULT_NOTEBOOK_FILE) {
                Ok(_yaml) => println!("Notebook saved"),
                Err(error) => {
                    println!("Unable to save notebook file: {}", DEFAULT_NOTEBOOK_FILE);
                    println!("{:?}", error);
                }
            };
        },
        Err(error) => {
            println!("Unable to load notebook file: {}", DEFAULT_NOTEBOOK_FILE);
            println!("{:?}", error);
        }
    };
}
