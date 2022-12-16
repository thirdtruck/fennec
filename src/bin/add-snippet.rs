use clap::{Parser, Subcommand};

use fennec::prelude::*;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    // Append the new snippet to the end of the notebook [default: prepend]
    #[arg(short, long)]
    append: bool,

    // Include this description with the new snippet
    #[arg(short, long)]
    description: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    Tunic,
    English,
}

fn main() {
    let args = Args::parse();

    println!("Loading notebook...");
    match notebook_from_yaml_file(DEFAULT_NOTEBOOK_FILE) {
        Ok((mut notebook, _yaml)) => {
            println!("Prepending snippet...");

            let starting_word = match &args.command {
                Some(Commands::Tunic) => vec![0x99].into(),
                Some(Commands::English) => "SomeEnglish".into(),
                None => vec![0x99].into(),
            };

            let words: Vec<Word> = vec![starting_word].into();

            let description = if let Some(description) = args.description {
                description
            } else {
                "ADD_DESCRIPTION_HERE".into()
            };

            let snippet = Snippet {
                words,
                source: Some(Source::ManualPageNumber(0)),
                description,
            };
                
            if args.append {
                notebook.snippets.push(snippet);
            } else {
                notebook.snippets.insert(0, snippet);
            }

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
