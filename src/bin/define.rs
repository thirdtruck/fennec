use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use fennec::prelude::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Word(WordCmd),
}

#[derive(Args)]
struct WordCmd {
    // The Tunic word to search for as a space-delimited sequence of integer values for its glyphs
    // Example: 1 27 339
    glyphs: Vec<u16>,
}

fn initialize_dictionary() {
    println!(
        "Initializing dictionary file: {}...",
        DEFAULT_DICTIONARY_FILE
    );

    let english_word: Word = "example".into();
    let tunic_word: Word = vec![DEFAULT_GLYPH, DEFAULT_GLYPH, DEFAULT_GLYPH].into();

    let dictionary = Dictionary::new()
        .with_new_definition(&english_word, "An example English word entry".into())
        .with_annotation(&english_word, "Example Note".into())
        .with_new_definition(&tunic_word, "An example Tunic word entry".into())
        .with_annotation(&tunic_word, "Example Note".into());

    let yaml = serde_yaml::to_string(&dictionary).expect("Unable to serialize entry");
    dictionary_to_yaml_file(&dictionary, DEFAULT_DICTIONARY_FILE).expect("Unable to save file");

    println!("YAML output:");
    println!("{}", yaml);

    println!("Initialized dictionary file");
}

fn search_for_word(cmd: WordCmd) {
    let word: Word = cmd.glyphs.into();
    let readable_word: String = format_word_for_reading(&word);

    println!("Loading dictionary...");

    match dictionary_from_yaml_file(DEFAULT_DICTIONARY_FILE) {
        Ok((dictionary, _yaml)) => {
            println!(
                "Searching the dictionary for word {} ...",
                readable_word.green()
            );

            if let Some(entry) = dictionary.get(&word) {
                let definition: String = entry.definition().clone().unwrap_or("[Undefined]".into());

                println!("-----");
                println!("  {}: {}", readable_word.green().bold(), definition.bold());
                println!();
                println!("  Notes:");
                for note in entry.notes().iter() {
                    println!("    - {}", note.as_text());
                }
                println!("-----");
            } else {
                println!("Word not found");
            }
        }
        Err(error) => {
            println!(
                "Unable to load dictionary file: {}",
                DEFAULT_DICTIONARY_FILE
            );
            println!("{:?}", error);
        }
    };
}

fn format_word_for_reading(word: &Word) -> String {
    match &word.word_type {
        WordType::Tunic(TunicWord { glyphs, .. }) => glyphs
            .iter()
            .map(|glyph| glyph.0.to_string())
            .reduce(|word, glyph_value| word + " " + &glyph_value)
            .map_or("(Empty)".into(), |word| format!("[{}]", word)),
        WordType::English(word) => word.text(),
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => initialize_dictionary(),
        Commands::Word(cmd) => search_for_word(cmd),
    }
}
