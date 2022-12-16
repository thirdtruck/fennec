use clap::{Args, Parser, Subcommand};

use fennec::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    // Append the new snippet to the end of the notebook [default: prepend]
    #[arg(short, long)]
    append: bool,

    // Include this description with the new snippet
    #[arg(short, long)]
    description: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Include a Tunic word
    #[command(subcommand)]
    Tunic(Tunic),

    // Include an English word
    English(English),
}

#[derive(Subcommand)]
enum Tunic {
    // Manual page number
    Page(Page),

    // Screenshot filename
    Screenshot(Screenshot),

    // Other source
    Other(Other),
}

#[derive(Args)]
struct Page {
    number: usize,
}

#[derive(Args)]
struct English {
    string: String,
}

#[derive(Args)]
struct Screenshot {
    filename: String,
}

#[derive(Args)]
struct Other {
    string: String,
}

fn main() {
    let cli = Cli::parse();

    println!("Loading notebook...");
    match notebook_from_yaml_file(DEFAULT_NOTEBOOK_FILE) {
        Ok((mut notebook, _yaml)) => {
            println!("Prepending snippet...");

            let snippet = match &cli.command {
                Some(Commands::Tunic(args)) => tunic_word_snippet(args),
                Some(Commands::English(args)) => english_word_snippet(args),
                None => panic!("Missing command"), // Make this required by definition
            };

            let description = if let Some(description) = cli.description {
                description
            } else {
                "ADD_DESCRIPTION_HERE".into()
            };

            let snippet = Snippet {
                description,
                ..snippet
            };

            println!("New snippet: {:?}", snippet);
                
            if cli.append {
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

fn english_word_snippet(args: &English) -> Snippet {
    Snippet {
        words: vec![args.string.clone().into()],
        source: Some(Source::Other("WIP".into())),
        description: "WIP".into(),
    }
}

fn tunic_word_snippet(args: &Tunic) -> Snippet {
    let word = vec![0x99].into();
    let words = vec![word].into();

    let source = Some(match args {
        Tunic::Page(page) => Source::ManualPageNumber(page.number),
        Tunic::Screenshot(screenshot) => Source::ScreenshotFilename(screenshot.filename.clone()),
        Tunic::Other(other) => Source::Other(other.string.clone()),
    });

    Snippet {
        words,
        source,
        description: "WIP".into(),
    }
}
