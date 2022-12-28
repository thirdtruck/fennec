use clap::{Args, Parser, Subcommand};
use colored::{ColoredString, Colorize};

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

fn search_snippets(notebook: Notebook, search_args: Snippets) {
    let word = search_args.word.expect("Missing argument: glyph values for word");

    println!("Looking for word {}", word.green());

    let glyphs: Vec<Glyph> = word
        .split_whitespace()
        .map(|string| {
            u16::from_str_radix(string, 10)
                .expect("Invalid glyph value. Expected a base 10 u16 value")
                .into()
        })
        .collect();

    let word: Word = glyphs.into();

    let matches: Vec<&Snippet> = notebook
        .snippets
        .iter()
        .filter(|snippet| snippet.contains_word(&word))
        .collect();

    println!("Found {} match(es)", matches.len());

    for (index, snippet) in matches.iter().enumerate() {
        let source = snippet
            .source
            .clone()
            .map_or("(None)".into(), |source| source.to_string());

        let sentence: Vec<ColoredString> = snippet
            .words
            .iter()
            .map(|w| (format_word_for_reading(w), *w == word))
            .map(|(w, matches)| if matches { w.green() } else { w.normal() })
            .collect();

        println!(" {:3}: {}", index, snippet.description.green().bold());

        println!("      {}", source);

        print!("      ");
        for word in sentence {
            print!("{}", word);
            print!(" ");
        }
        println!();

        println!();
    }
}

fn format_word_for_reading(word: &Word) -> String {
    match word {
        Word::Tunic(glyphs) => {
            glyphs
                .iter()
                .map(|glyph| glyph.0.to_string())
                .reduce(|word, glyph_value| word + " " + &glyph_value)
                .map_or("(Empty)".into(), |word| format!("[{}]", word))
        }
        Word::English(text) => text.to_string(),
    }
}
