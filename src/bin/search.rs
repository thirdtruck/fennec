use clap::{Args, Parser, Subcommand};
use colored::{ColoredString, Colorize};
use std::collections::HashMap;

use fennec::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search through every snippet in the notebook
    Snippets(Snippets),
    /// Search for usage
    Usage(Usage),
}

#[derive(Args)]
struct Snippets {
    word: Option<String>,
    /// Render words as their definition if available. Default: Render words as their glyph values
    #[arg(short, long)]
    define_inline: bool,
}

#[derive(Args)]
struct Usage {
    words: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("Loading notebook...");

    let (notebook, _yaml) = notebook_from_yaml_file(DEFAULT_NOTEBOOK_FILE)
        .unwrap_or_else(|error| {
            println!("Unable to load notebook file: {}", DEFAULT_NOTEBOOK_FILE);
            println!("{:?}", error);
            panic!("Search aborted");
        });

    let (dictionary, _yaml) = dictionary_from_yaml_file(DEFAULT_DICTIONARY_FILE)
        .unwrap_or_else(|error| {
            println!("Unable to load dictionary file: {}", DEFAULT_NOTEBOOK_FILE);
            println!("{:?}", error);
            panic!("Search aborted");
        });

    println!("Searching...");

    match cli.command {
        Commands::Snippets(args) => search_snippets(notebook, dictionary, args),
        Commands::Usage(args) => search_usage(notebook, args),
    };
}

fn search_usage(notebook: Notebook, usage_args: Usage) {
    let usage_type = usage_args
        .words
        .expect("Missing argument: type of usage to search");

    match usage_type.as_str() {
        "words" => search_word_usage(notebook),
        _ => panic!("Unsupported usage type: {}", usage_type),
    };
}

fn search_word_usage(notebook: Notebook) {
    println!("Search word usage...");

    let mut usage_counts: HashMap<Word, usize> = HashMap::new();

    for snippet in notebook.snippets.iter() {
        for word in snippet.words.iter() {
            if let WordType::Tunic(word) = &word.word_type {
                let word: Word = word.clone().into();

                if let Some(count) = usage_counts.get_mut(&word) {
                    *count = *count + 1;
                } else {
                    usage_counts.insert(word, 1);
                }
            }
        }
    }

    let mut usage_counts: Vec<(Word, usize)> = usage_counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    usage_counts.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in usage_counts {
        println!("{:4} -> {}", count, format_word_for_reading_as_glyphs(&word));
    }
}

fn search_snippets(notebook: Notebook, dictionary: Dictionary, search_args: Snippets) {
    let word = search_args
        .word
        .expect("Missing argument: glyph values for word");

    let define_inline = search_args.define_inline;

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
            .map(|w| {
                let formatted_word = if define_inline {
                    format_word_for_reading_as_defined(&dictionary, w)
                } else {
                    format_word_for_reading_as_glyphs(w)
                };

                (formatted_word, *w == word)
            })
            .map(|(w, matches)| if matches { w.underline().green() } else { w })
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

fn format_word_for_reading_as_defined(dictionary: &Dictionary, word: &Word) -> ColoredString {
    match &word.word_type {
        WordType::Tunic(word) => {
            if let Some(entry) = dictionary.get(word) {
                match entry.definition() {
                    Definition::Undefined => format_glyphs_for_reading(word.glyphs()).normal(),
                    Definition::Tentative(text) => format!("{}", text).bright_yellow().underline(),
                    Definition::Confirmed(text) => format!("{}", text).bright_yellow(),
                }
            } else {
                format_glyphs_for_reading(word.glyphs()).normal()
            }
        }
        WordType::English(word) => word.text().normal(),
    }
}

fn format_word_for_reading_as_glyphs(word: &Word) -> ColoredString {
    match &word.word_type {
        WordType::Tunic(word) => {
            format_glyphs_for_reading(word.glyphs()).normal()
        }
        WordType::English(word) => word.text().normal(),
    }
}

fn format_glyphs_for_reading(glyphs: Vec<Glyph>) -> String {
    glyphs
        .iter()
        .map(|glyph| glyph.0.to_string())
        .reduce(|word, glyph_value| word + " " + &glyph_value)
        .map_or("(Empty)".into(), |word| format!("[{}]", word))
}
