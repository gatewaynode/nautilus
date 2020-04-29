// This is a simple CLI CMS for managing blog like content in a PostgreSQL database.  Be aware this
// is a learing project I have created while teaching myself Rust and is by no means idiomatic or
// even high quality, it is really just basically working in a language I barely know.
//
// Copyright 2020 Gatewaynode
// Distributed under the terms of the GPL version 3

#[macro_use] extern crate prettytable;

extern crate clap;
extern crate dotenv;
extern crate subprocess;
extern crate tempfile;
extern crate diesel;

use tempfile::NamedTempFile;
use std::fs;
use nautilus::*;
use std::{thread, time};
use std::io::prelude::*;
use std::process::exit;
use self::models::{Post, NewPost, NewLink};
use prettytable::{Table};
use serde_json::json;
use rustyline::error::ReadlineError;
use rustyline::Editor;


// @TODO Edit should take field types as arguments or open interactive choice of fields to edit
// @TODO Add post_id checking standard handler (currently just fails if the post_id doesn't exist)
// @TODO Default mode should be interactive if no args or subcommands are received
// @TODO Need the import function to handle insert new as well as update existing
// @TODO Navigation content type (maybe just use links tagged nav????)

/// The command line CMS
///
/// Two basic content types supported in the library and app with the following fields
/// available:
/// * Post
///   - ID: i32, auto
///   - Publised: Bool, default "false"
///   - Title: String
///   - Body: String
///   - Time: Datetime+TZ, Auto
///   - Tags: String, default "front"
/// * Link
///   - ID: i32, auto
///   - Publised: Bool, default "false"
///   - Text: String
///   - Title: String
///   - URL: String
///   - Tags: String
///   - Time: Datetime+TZ, Auto
///
struct State {
    verbose: bool,
    debug: bool,
    edit_title: bool,
    edit_body: bool,
    edit_tags: bool,
    edit_summary: bool,
    // interactive: bool, // Just thinking out loud
    // connection: SomeConnectionPool,
}

impl State {
    fn new() -> State {
        State {
            verbose: false,
            debug: false,
            edit_title: false,
            edit_body: false,
            edit_tags: false,
            edit_summary: false,
        }
    }
}

// Setup initial state and parse args to modify state or trigger functions
fn main() {
    let mut state = State::new();
    use clap::{load_yaml, App};

    let yaml = load_yaml!("cli.yml");

    let matches = App::from(yaml).get_matches();

    // This will work for arg flags
    if matches.is_present("verbose") {
        state.verbose = true;
    }
    if matches.is_present("debug") {
        state.debug = true;
    }

    match matches.subcommand() {
        ("list", Some(_clone_matches)) => {
            list_posts(state)
        }
        ("show", Some(_clone_matches)) => {
            // UGLY get's the subcommand arg, unwraps it, parses it as i32, unwraps that or on fail gives it a value of 1
            // Rinse, repeat, soak eyes in bleach
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(0);
            show_post(state, this_post)
        }
        ("post", Some(_clone_matches)) => {
            write_post(state)
        }
        ("epost", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(0);
            if _clone_matches.is_present("title") {
                state.edit_title = true;
            }
            if _clone_matches.is_present("body") {
                state.edit_body = true;
            }
            if _clone_matches.is_present("tags") {
                state.edit_tags = true;
            }
            if _clone_matches.is_present("summary") {
                state.edit_summary = true;
            }
            edit_post(state, this_post)
        }
        ("depost", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(0);
            delete_a_post(state, this_post)
        }
        ("link", Some(_clone_matches)) => {
            write_link(state)
        }
        ("delink", Some(_clone_matches)) => {
            let link_id = _clone_matches.value_of("link_id").unwrap().parse::<i32>().unwrap_or(0);
            delete_a_link(state, link_id)
        }
        ("export", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(0);
            let export_filename = _clone_matches.value_of("export_filename").unwrap();
            export_post(state, this_post, export_filename)
        }
        ("import", Some(_clone_matches)) => {
            // println!("NOT IMPLEMENTED!  Import new post from filename {:#?}", _clone_matches.value_of("import_filename"))
            let import_filename = _clone_matches.value_of("import_filename").unwrap();
            import_post(state, import_filename)
        }
        ("testing", Some(_clone_matches)) => {
            let output = input("promted$: ");
            println!("output = {:?}", output)
        }
        ("", None) => println!("No subcommand used"), // @TODO add a default action here
        _ => unreachable!(),
    }

}

// <-- Helper Functions -->
fn input(prompt: &str) -> String {
    let mut editor = Editor::<()>::new();
    // @TODO get history working
    // if editor.load_history("~/.n4_history.txt").is_err() {
    //     println!("No previous history found.")
    // }
    let readline = editor.readline(&prompt);
    match readline {
        Ok(line) => {
            // editor.add_history_entry(line.as_str());
            line
        },
        Err(ReadlineError::Interrupted) => {
            exit(0)
        },
        Err(ReadlineError::Eof) => {
            exit(1)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1)
        }
    }
}

fn edit(prompt: &str, value: &str) -> String {
    let mut editor = Editor::<()>::new();
    let readline = editor.readline_with_initial(prompt, (value, ""));
    match readline {
        Ok(line) => {
            line
        },
        Err(ReadlineError::Interrupted) => {
            exit(0)
        },
        Err(ReadlineError::Eof) => {
            exit(1)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1)
        }
    }
}

// <-- Primary functions -->
fn list_posts(state: State) {
    let all_posts: Vec<Post> = read_all_posts();
    if state.verbose {
        println!("Displaying {} posts:", all_posts.len());
    }

    let mut table = Table::new();
    table.add_row(row!["ID", "TITLE", "TAGS", "TIME"]);
    for post in all_posts {
        table.add_row(row![&post.id, &post.title, &post.tags, &post.time]);
    }
    table.printstd();
}

fn write_post(state: State) {
    if state.verbose {
        println!("Writing Post.");
    }
    let raw_title = input("Title: ");

    let some_file = NamedTempFile::new();
    let file_path = String::from(some_file.unwrap().path().to_string_lossy());
    if state.verbose {
        println!("Writing Body...");
        let delay = time::Duration::from_millis(750);
        thread::sleep(delay);
    }

    subprocess::Exec::cmd("vim")
        .arg(&file_path)
        .join()
        .unwrap();

    let contents = fs::read_to_string(&file_path)
        .expect("Something went wrong reading the file");

    fs::remove_file(&file_path)
        .expect("Could not cleanup temp file");

    let raw_tags = input("Tags: ");
    let raw_summary = input("Summary: ");

    let rawpost = NewPost {
        title: &raw_title,
        body: &contents,
        tags: &raw_tags,
        summary: &raw_summary,
    };

    let post = create_post(&rawpost);
    if state.verbose {
        println!("\nSaved {} with id {}", &rawpost.title, post.id);
    }
}

fn edit_post(mut state: State, post_id: i32) {
    if state.verbose {
        println!("Editing post {}", post_id);
    }
    if !state.edit_title && !state.edit_body && !state.edit_tags && !state.edit_summary {
        if state.verbose {
            println!("No fields selected, assuming body...");
        }
        state.edit_body = true;
    }


    let current_content: Post = read_post(post_id);

    let mut raw_title: String = current_content.title.clone();
    let mut raw_body: String = current_content.body.clone();
    let mut raw_tags: String = current_content.tags.clone();
    let mut raw_summary: String = current_content.summary.clone();

    if state.edit_title {
        raw_title = edit("Edit title: ", &raw_title);
    }
    if state.edit_body {
        let some_file = NamedTempFile::new();
        let file_path = String::from(some_file.unwrap().path().to_string_lossy());
        fs::write(&file_path, current_content.body)
            .expect("Something went wrong with writing the temp file");

        subprocess::Exec::cmd("vim")
            .arg(&file_path)
            .join()
            .unwrap();

        raw_body = fs::read_to_string(&file_path).unwrap();
        // let updated_post = Post {
        //     body: fs::read_to_string(&file_path).unwrap(),
        //     ..current_content
        // };
    }
    if state.edit_tags {
        raw_tags = edit("Edit tags: ", &raw_tags);
    }
    if state.edit_summary {
        raw_summary = edit("Edit summary: ", &raw_summary);
    }

    let edited_content = Post {
        title: raw_title,
        body: raw_body,
        tags: raw_tags,
        summary: raw_summary,
        ..current_content
    };

    // let result = update_post(&updated_post).unwrap();
    let result = update_post(&edited_content).unwrap();

    if state.verbose {
        println!("Update post result: {}", &result);
    }

}

fn show_post(state: State, post_id: i32) {
    if state.verbose {
        println!("Showing post {}", post_id);
    }

    let output: Post = read_post(post_id);

    let mut table = Table::new();
    table.add_row(row!["ID", "TITLE", "BODY", "PUBLISHED"]);
    table.add_row(row![output.id, output.title, output.body, output.published]);
    table.printstd();
}

fn delete_a_post(state: State, post_id: i32) {
    if state.verbose {
        println!("Deleting post {}", post_id);
    }
    delete_post(post_id)
}

fn write_link(state: State) {
    if state.verbose {
        println!("Writing Link");
    }
    let raw_text = input("Display text: ");
    let raw_title = input("Hover title: ");
    let raw_url = input("Link URL: ");
    let raw_tags = input("Tags: ");

    let rawlink = NewLink {
        text: &raw_text,
        title: &raw_title,
        url: &raw_url,
        tags: &raw_tags,
    };

    let link = create_link(&rawlink);
    if state.verbose {
        println!("\nSaved {} with id {}", &link.text, &link.id);
    }

}

fn delete_a_link(state: State, link_id: i32) {
    if state.verbose {
        println!("Deleting link {}", link_id);
    }
    delete_link(link_id)
}

fn export_post(state: State, this_post: i32, export_filename: &str) {
    if state.verbose {
        println!("Exporting post ID {} to filename {}", &this_post, &export_filename);
    }
    let post_to_export = read_post(this_post);

    let post_json = json!({
        "id": &post_to_export.id,
        "published": &post_to_export.published,
        "title": &post_to_export.title,
        "body": &post_to_export.body,
    });

    if state.verbose {
        println!("JSON out: {}", post_json.to_string());
    }
    let mut file = fs::File::create(&export_filename)
        .expect("Problem creating export file");
    file.write_all(post_json.to_string().as_bytes())
        .expect("Could not write to export file");

}

fn import_post(state: State, import_filename: &str) {
    if state.verbose {
        println!("Importing filename {} as a piece of Post content", &import_filename);
    }
    let file_string = fs::read_to_string(import_filename)
        .expect("Could not open the import filename");
    let imported_post: Post = serde_json::from_str(&file_string).unwrap();

    let result = update_post(&imported_post).unwrap();
    if state.verbose {
        println!("Imported post result: {}", &result)
    }
}
