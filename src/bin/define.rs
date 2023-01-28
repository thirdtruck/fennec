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
    /// Create a new dictionary file. (Default: dictionary.yaml)
    Init,
    /// Search for a word's definition
    Word(WordCmd),
    /// Add a new definition
    Add(AddCmd),
}

#[derive(Args)]
struct WordCmd {
    /// The Tunic word to search for as a space-delimited sequence of integer values for its glyphs
    /// Example: 1 27 339
    glyphs: Vec<u16>,
}

#[derive(Args)]
struct AddCmd {
    /// The Tunic word to be defined as a space-delimited sequence of integer values for its glyphs
    /// Example: 1 27 339
    glyphs: Vec<u16>,

    /// The new word's definition
    #[arg(short, long)]
    definition: String,

    /// Attach a note to the new defition. Supports multiple uses.
    #[arg(short, long, action = clap::ArgAction::Append, num_args(1))]
    note: Vec<String>,

    /// Set the definition type. Options: tenative, confirmed, undefined. Default: tentative.
    #[arg(short, long, name = "DEFINITION TYPE")]
    _type: Option<String>,
}


fn initialize_dictionary() {
    println!(
        "Initializing dictionary file: {}...",
        DEFAULT_DICTIONARY_FILE
    );

    let tunic_word: TunicWord = vec![DEFAULT_GLYPH, DEFAULT_GLYPH, DEFAULT_GLYPH].into();

    let dictionary = Dictionary::new()
        .with_new_definition(&tunic_word, "An example Tunic word entry".into())
        .with_annotation(&tunic_word, "Example Note".into());

    let yaml = serde_yaml::to_string(&dictionary).expect("Unable to serialize entry");
    dictionary_to_yaml_file(&dictionary, DEFAULT_DICTIONARY_FILE).expect("Unable to save file");

    println!("YAML output:");
    println!("{}", yaml);

    println!("Initialized dictionary file");
}

fn search_for_word(cmd: WordCmd) {
    let word: TunicWord = cmd.glyphs.into();
    let dict_word: DictionaryWord = (&word).into();
    let readable_word: String = format_word_for_reading(&word);

    println!("Loading dictionary...");

    match dictionary_from_yaml_file(DEFAULT_DICTIONARY_FILE) {
        Ok((dictionary, _yaml)) => {
            println!(
                "Searching the dictionary for word {} ...",
                readable_word.green()
            );

            if let Some(entry) = dictionary.get(&dict_word) {
                let definition: String = match entry.definition() {
                    Definition::Undefined => "Undefined".into(),
                    Definition::Tentative(text) => text.clone(),
                    Definition::Confirmed(text) => text.clone(),
                };

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

fn add_definition(args: AddCmd) {
    let definition = args.definition;

    let def_type = args._type.unwrap_or("tentative".to_owned());

    let definition = match def_type.as_ref() {
        "tentative" => Definition::Tentative(definition.to_owned()),
        "confirmed" => Definition::Confirmed(definition.to_owned()),
        "undefined" => Definition::Undefined,
        _ => panic!("Unrecognized definition type: {def_type}"),
    };

    let notes = args
        .note
        .iter()
        .map(|text| Note(text.to_string()))
        .collect::<Vec<Note>>();

    let entry = Entry::new(definition, notes);

    let word: TunicWord = args.glyphs.into();
    let word: DictionaryWord = word.into();

    println!("Loading dictionary...");

    match dictionary_from_yaml_file(DEFAULT_DICTIONARY_FILE) {
        Ok((dictionary, _yaml)) => {
            println!("Adding definition...");

            match dictionary.get(&word) {
                None => {
                    let dictionary = dictionary.with_new_complete_definition(&word, &entry);
                    dictionary_to_yaml_file(&dictionary, DEFAULT_DICTIONARY_FILE).expect("Unable to save file");
                    println!("Definition added for {word}: {entry}");
                },
                Some(_existing_def) => panic!("A definition already exists for {word}. Aborting"),
            };
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

fn format_word_for_reading(word: &TunicWord) -> String {
    word
        .glyphs()
        .iter()
        .map(|glyph| glyph.0.to_string())
        .reduce(|word, glyph_value| word + " " + &glyph_value)
        .map_or("(Empty)".into(), |word| format!("[{}]", word))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(cmd) => add_definition(cmd),
        Commands::Init => initialize_dictionary(),
        Commands::Word(cmd) => search_for_word(cmd),
    }
}
