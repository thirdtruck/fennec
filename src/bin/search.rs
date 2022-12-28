use clap::{Args, Parser, Subcommand};

use fennec::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Search through every snippet in the notebook
    Snippets(Snippets),
}

#[derive(Args)]
struct Snippets {
    #[arg(short, long)]
    word: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("Loading notebook...");
    match notebook_from_yaml_file(DEFAULT_NOTEBOOK_FILE) {
        Ok((notebook, _yaml)) => {
            println!("Searching...");

            match cli.command {
                Commands::Snippets(args) => search_snippets(notebook, args),
            };
        }
        Err(error) => {
            println!("Unable to load notebook file: {}", DEFAULT_NOTEBOOK_FILE);
            println!("{:?}", error);
        }
    };
}

fn search_snippets(_notebook: Notebook, _search_args: Snippets) {
}
