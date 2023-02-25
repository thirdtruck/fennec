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
    /// Find all snippets for a given manual page
    Page(Page),
    /// List all entries
    #[command(subcommand)]
    List(List),
}

#[derive(Args)]
struct Page {
    /// Manual page number
    number: usize,
    /// Render words as their definition if available. Default: Render words as their glyph values
    #[arg(short, long)]
    define_inline: bool,
}

#[derive(Args)]
struct Snippets {
    /// Search for snippets that contain this Tunic word. Space-separated list of glyph values
    word: Option<Vec<u16>>,
    /// Render words as their definition if available. Default: Render words as their glyph values
    #[arg(short, long)]
    define_inline: bool,
    /// Use this as the definition of the word being searched for
    #[arg(short, long)]
    as_if: Option<String>,
}

#[derive(Args)]
struct Usage {
    words: Option<String>,
}

#[derive(Subcommand)]
enum List {
    /// List all snippets
    Snippets(ListSnippets),
}

#[derive(Args)]
struct ListSnippets {
    #[arg(short, long)]
    define_inline: bool,
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
        Commands::Page(args) => search_by_page(notebook, dictionary, args),
        Commands::List(subcommand) => {
            match subcommand {
                List::Snippets(args) => list_all_snippets(notebook, dictionary, args),
            }
        }
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

fn search_by_page(notebook: Notebook, dictionary: Dictionary, args: Page) {
    let page_number = args.number;
    let define_inline = args.define_inline;

    println!("Searching for all snippets on manual page {}", page_number);

    let matches: Vec<&Snippet> = notebook
        .snippets
        .iter()
        .filter(|snippet| {
            match &snippet.source {
                Some(source) => match &source {
                    Source::ManualPageNumber(number) => *number == page_number,
                    _ => false
                },
                None => false,
            }
        })
        .collect();

    // TODO: Refactor print_snippet to slash that argument count. May involve callbacks
    let placeholder_word: Word = vec![0].into();

    for (index, snippet) in matches.iter().enumerate() {
        print_snippet(snippet, index, define_inline, Some(&placeholder_word), &dictionary);
    }
}

fn search_snippets(notebook: Notebook, dictionary: Dictionary, search_args: Snippets) {
    let word = search_args
        .word
        .expect("Missing argument: glyph values for word");

    let word: Word = word.into();

    let define_inline = search_args.define_inline || search_args.as_if.is_some();

    println!("Looking for word {}", word);

    let dictionary = if let Some(temporary_definition) = &search_args.as_if {
        if let WordType::Tunic(tunic_word) = &word.word_type {
            dictionary.with_new_definition(&tunic_word, temporary_definition.clone())
        } else {
            dictionary
        }
    } else {
        dictionary
    };

    let matches: Vec<&Snippet> = notebook
        .snippets
        .iter()
        .filter(|snippet| snippet.contains_word(&word))
        .collect();

    println!("Found {} match(es)", matches.len());

    for (index, snippet) in matches.iter().enumerate() {
        print_snippet(snippet, index, define_inline, Some(&word), &dictionary);
    }
}

fn list_all_snippets(notebook: Notebook, dictionary: Dictionary, args: ListSnippets) {
    let define_inline = args.define_inline;

    for (index, snippet) in notebook.snippets.iter().enumerate() {
        print_snippet(snippet, index, define_inline, None, &dictionary);
    }
}

fn print_snippet(snippet: &Snippet, index: usize, define_inline: bool, selected_word: Option<&Word>, dictionary: &Dictionary) {
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

            let does_match = selected_word.map(|word| *w == *word).unwrap_or(false);

            (formatted_word, does_match)
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

fn format_word_for_reading_as_defined(dictionary: &Dictionary, word: &Word) -> ColoredString {
    match &word.word_type {
        WordType::Tunic(word) => {
            let dict_word: DictionaryWord = word.into();
            if let Some(entry) = dictionary.get(&dict_word) {
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
