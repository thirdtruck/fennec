#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::State;
use rocket_dyn_templates::{Template};
use serde::Serialize;

use fennec::prelude::*;

#[derive(Clone, Serialize)]
struct AppContext<'a> {
    title: &'a str,
    appname: &'a str,
}

#[derive(Clone, Serialize)]
struct DefinitionsContext<'a> {
    app: AppContext<'a>,
    entries: Vec<DictionaryEntry>,
}

#[derive(Clone, Serialize)]
struct SnippetsContext<'a> {
    app: AppContext<'a>,
    snippets: Vec<SnippetRow>,
}

#[derive(Clone, Serialize)]
struct RootContext<'a> {
    app: AppContext<'a>,
    dictionary: Dictionary,
    notebook: Notebook,
}

#[derive(Clone, Serialize)]
struct SnippetRow {
    source: String,
    description: String,
    transcribed: bool,
    notes: Vec<Note>,
    words: Vec<WordRow>,
}

#[derive(Clone, Serialize)]
struct WordRow {
    word_type: String,
    is_tunic: bool,
    is_english: bool,
    has_definition: bool,
    text: String,
    glyphs: Vec<Glyph>,
    has_border: bool,
    colored: bool,
}

#[derive(Clone, Serialize)]
struct DictionaryEntry {
    glyphs: Vec<Glyph>,
    definition: String,
    notes: Vec<String>,
}

#[get("/")]
fn index(state: &State<RootContext>) -> Template {
    Template::render("index", state.app.clone())
}

#[get("/snippets")]
fn snippets(state: &State<RootContext>) -> Template {
    let snippets: Vec<SnippetRow> = state
        .notebook
        .snippets
        .iter()
        .map(|snip| {
            let source = match &snip.source {
                Some(Source::ManualPageNumber(page_number)) => format!("/media/manual_pages/page{:0>2}.jpg", page_number),
                Some(Source::ScreenshotFilename(file)) => format!("/media/screenshots/{file}"),
                Some(Source::Other(_)) => "/media/404".to_owned(),
                None => "/media/404".to_owned(),
            };

            let description = snip.description.clone();

            let transcribed = snip.transcribed;

            let notes = snip.notes.clone();

            let words: Vec<WordRow> = snip
                .words
                .iter()
                .map(|word| {
                    match &word.word_type {
                        WordType::English(english_word) => WordRow {
                            word_type: "English".to_owned(),
                            is_tunic: false,
                            is_english: true,
                            has_definition: true,
                            text: english_word.text(),
                            glyphs: vec![],
                            has_border: false,
                            colored: false,
                        },
                        WordType::Tunic(tunic_word) => {
                            let (has_definition, definition) = state
                                .dictionary
                                .get(&tunic_word.into())
                                .map(|entry| {
                                    let definition = match entry.definition() {
                                        Definition::Undefined => "[Undefined]".to_owned(),
                                        Definition::Tentative(text) => text.clone(),
                                        Definition::Confirmed(text) => text.clone(),
                                    };

                                    (true, definition)
                                })
                                .unwrap_or((false, "".to_owned()));

                            WordRow {
                                word_type: "Tunic".to_owned(),
                                is_tunic: true,
                                is_english: false,
                                has_definition,
                                text: definition,
                                glyphs: tunic_word.glyphs(),
                                has_border: tunic_word.has_border(),
                                colored: tunic_word.colored(),
                            }
                        }
                    }
                })
                .collect();

            SnippetRow {
                source,
                description,
                transcribed,
                notes,
                words,
            }
        })
        .collect();

    let context = SnippetsContext {
        app: state.app.clone(),
        snippets,
    };

    Template::render("snippets", context)
}

#[get("/definitions")]
fn definitions(state: &State<RootContext>) -> Template {
    let context = DefinitionsContext {
        app: state.app.clone(),
        entries: to_dictionary_entries(&state.dictionary),
    };

    Template::render("definitions", context)
}

fn to_dictionary_entries(dictionary: &Dictionary) -> Vec<DictionaryEntry> {
     dictionary
        .entries()
        .iter()
        .map(|(word, entry)| {
            let glyphs = word.glyphs();
            let definition =  match entry.definition() {
                Definition::Undefined => "[Undefined]".to_owned(),
                Definition::Tentative(text) => text.clone(),
                Definition::Confirmed(text) => text.clone(),
            };
            let notes: Vec<String> = entry
                .notes()
                .iter()
                .map(|n| n.into())
                .collect();

            DictionaryEntry {
                glyphs,
                definition,
                notes,
            }
        })
        .collect()
}

#[launch]
fn rocket() -> _ {
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

    let root_context = RootContext {
        app: AppContext {
            title: "Fennec",
            appname: "Fennec",
        },
        notebook,
        dictionary,
    };

    rocket::build()
        .mount("/", routes![index, definitions, snippets])
        .mount("/media", FileServer::from(relative!("sources")))
        .manage(root_context)
        .attach(Template::fairing())
}
