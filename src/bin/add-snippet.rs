use clap::{Args, Parser, Subcommand};

use fennec::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    // Append the new snippet to the end of the notebook [default: prepend]
    #[arg(short, long)]
    append: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Include a Tunic word
    #[command(subcommand)]
    Tunic(Tunic),

    // Include an English word
    #[command(subcommand)]
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

#[derive(Subcommand)]
enum English {
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
    description: String,
    // Required for English words
    word_text: Option<String>,
}

#[derive(Args)]
struct Screenshot {
    filename: String,
    description: String,
    // Required for English words
    word_text: Option<String>,
}

#[derive(Args)]
struct Other {
    text: String,
    description: String,
    // Required for English words
    word_text: Option<String>,
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
        }
        Err(error) => {
            println!("Unable to load notebook file: {}", DEFAULT_NOTEBOOK_FILE);
            println!("{:?}", error);
        }
    };
}

fn english_word_snippet(args: &English) -> Snippet {
    let (source, description, word_text) = match args {
        English::Page(page) => (
            Source::ManualPageNumber(page.number),
            page.description.clone(),
            page.word_text.clone(),
        ),
        English::Screenshot(screenshot) => (
            Source::ScreenshotFilename(screenshot.filename.clone()),
            screenshot.description.clone(),
            screenshot.word_text.clone(),
        ),
        English::Other(other) => (
            Source::Other(other.text.clone()),
            other.description.clone(),
            other.word_text.clone(),
        ),
    };

    let word = match word_text {
        Some(word_text) => word_text.into(),
        // TODO: Make this required more gracefully via clap
        None => panic!("Missing text argument"),
    };
    let words = vec![word];

    let source = Some(source);

    Snippet {
        words,
        source,
        description,
    }
}

fn tunic_word_snippet(args: &Tunic) -> Snippet {
    let word = vec![0x99].into();
    let words = vec![word].into();

    let (source, description) = match args {
        Tunic::Page(page) => (
            Source::ManualPageNumber(page.number),
            page.description.clone(),
        ),
        Tunic::Screenshot(screenshot) => (
            Source::ScreenshotFilename(screenshot.filename.clone()),
            screenshot.description.clone(),
        ),
        Tunic::Other(other) => (Source::Other(other.text.clone()), other.description.clone()),
    };
    let source = Some(source);

    Snippet {
        words,
        source,
        description,
    }
}
