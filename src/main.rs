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

// use tempfile::NamedTempFile;
use std::fs;
use nautilus::*;
use std::{thread, time};
use std::io::prelude::*;
// use std::process::exit;
use self::models::{Post, NewPost, Link, NewLink, System, NewSystem};
use prettytable::{Table};
use serde_json::json;
// use rustyline::error::ReadlineError;
// use rustyline::Editor;
use simple_prompts::{edit_prompt, prompt};
use vim_edit::{vim_create, vim_edit};


// @TODO Edit should take field types as arguments or open interactive choice of fields to edit
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
    edit_text: bool,
    edit_url: bool,
    edit_all: bool,
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
            edit_text: false,
            edit_url: false,
            edit_all: false,
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
        ("slink", Some(_clone_matches)) => {
            show_links(state)
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
            if _clone_matches.is_present("all") {
                state.edit_all = true;
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
        ("elink", Some(_clone_matches)) => {
            let this_link = _clone_matches.value_of("link_id").unwrap().parse::<i32>().unwrap_or(0);
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
            if _clone_matches.is_present("all") {
                state.edit_all = true;
            }
            edit_link(state, this_link)
        }
        ("delink", Some(_clone_matches)) => {
            let link_id = _clone_matches.value_of("link_id").unwrap().parse::<i32>().unwrap_or(0);
            delete_a_link(state, link_id)
        }
        ("ssystem", Some(_clone_matches)) => {
            show_system(state)
        }
        ("system", Some(_clone_matches)) => {
            write_system(state)
        }
        ("esystem", Some(_clone_matches)) => {
            let system_key = String::from(_clone_matches.value_of("system_key").unwrap());
            edit_system(state, system_key)
        }
        ("dsystem", Some(_clone_matches)) => {
            let system_key = String::from(_clone_matches.value_of("system_key").unwrap());
            delete_a_system(state, system_key)
        }
        ("export", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(0);
            let export_filename = _clone_matches.value_of("export_filename").unwrap();
            export_post(state, this_post, export_filename)
        }
        ("import", Some(_clone_matches)) => {
            let import_filename = _clone_matches.value_of("import_filename").unwrap();
            import_post(state, import_filename)
        }
        ("testing", Some(_clone_matches)) => {
            let output = prompt("promted$: ");
            println!("output = {:?}", output)
        }
        ("", None) => println!("No subcommand used"), // @TODO add a default REPL action here
        _ => unreachable!(),
    }

}


// <-- Primary functions -->
fn list_posts(state: State) {
    let all_posts: Vec<Post> = read_all_posts();
    if state.verbose {
        println!("Displaying {} posts:", all_posts.len());
    }

    let mut table = Table::new();
    table.add_row(row!["ID", "TITLE", "SUMMARY", "TAGS", "TIME"]);
    for post in all_posts {
        table.add_row(row![&post.id, &post.title, &post.summary, &post.tags, &post.time]);
    }
    table.printstd();
}

fn show_links(state: State) {
    let all_links: Vec<Link> = read_all_links();
    if state.verbose {
        println!("Displaying {} links:", all_links.len());
    }

    let mut table = Table::new();
    table.add_row(row!["ID", "TEXT", "URL", "TITLE", "TIME"]);
    for link in all_links {
        table.add_row(row![&link.id, &link.text, &link.url, &link.title, &link.time]);
    }
    table.printstd();
}

fn write_post(state: State) {
    if state.verbose {
        println!("Writing Post.");
    }
    // let raw_title = input("Title: ");
    let raw_title = prompt("Title: ");
    if state.verbose {
        println!("Writing Body...");
        let delay = time::Duration::from_millis(750);
        thread::sleep(delay);
    }
    let raw_body = vim_create();
    let raw_tags = prompt("Tags: ");
    let raw_summary = prompt("Summary: ");

    let rawpost = NewPost {
        title: &raw_title,
        body: &raw_body,
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
    if !state.edit_title && !state.edit_body && !state.edit_tags && !state.edit_summary && !state.edit_all {
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

    if state.edit_title || state.edit_all {
        raw_title = edit_prompt("Edit title: ", &raw_title);
    }
    if state.edit_body || state.edit_all {
        raw_body = vim_edit(raw_body);
    }
    if state.edit_tags || state.edit_all {
        raw_tags = edit_prompt("Edit tags: ", &raw_tags);
    }
    if state.edit_summary || state.edit_all {
        raw_summary = edit_prompt("Edit summary: ", &raw_summary);
    }

    let edited_content = Post {
        title: raw_title,
        body: raw_body,
        tags: raw_tags,
        summary: raw_summary,
        ..current_content
    };

    let result = update_post(&edited_content).unwrap();

    if state.verbose {
        println!("Update post result: {:?}", &result);
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
    let raw_text = prompt("Display text: ");
    let raw_title = prompt("Hover title: ");
    let raw_url = prompt("Link URL: ");
    let raw_tags = prompt("Tags: ");

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

fn edit_link(mut state: State, link_id: i32) {
    if state.verbose {
        println!("Editing link {}", link_id);
    }
    if !state.edit_text && !state.edit_title && !state.edit_url && !state.edit_tags && !state.edit_all{
        if state.verbose {
            println!("No fields selected, assuming url...");
        }
        state.edit_url = true;
    }

    let current_content: Link = read_link(link_id);

    let mut raw_text: String = current_content.text.clone();
    let mut raw_title: String = current_content.title.clone();
    let mut raw_url: String = current_content.url.clone();
    let mut raw_tags: String = current_content.tags.clone();

    if state.edit_text || state.edit_all {
        raw_text = edit_prompt("Edit text: ", &raw_text);
    }
    if state.edit_title || state.edit_all {
        raw_title = edit_prompt("Edit title: ", &raw_title);
    }
    if state.edit_url || state.edit_all {
        raw_url = edit_prompt("Edit url: ", &raw_url);
    }
    if state.edit_tags || state.edit_all{
        raw_tags = edit_prompt("Edit tags: ", &raw_tags);
    }

    let edited_content = Link {
        text: raw_text,
        title: raw_title,
        url: raw_url,
        tags: raw_tags,
        ..current_content
    };

    let result = update_link(&edited_content).unwrap();

    if state.verbose {
        println!("Update post result: {:?}", &result);
    }
}

fn delete_a_link(state: State, link_id: i32) {
    if state.verbose {
        println!("Deleting link {}", link_id);
    }
    delete_link(link_id)
}

fn show_system(state: State) {
    let all_system: Vec<System> = read_all_system();
    if state.verbose {
        println!("Displaying {} system entries:", all_system.len());
    }

    let mut table = Table::new();
    table.add_row(row!["KEY", "DATA", "TIME"]);
    for system in all_system {
        table.add_row(row![&system.key, &system.data, &system.time]);
    }
    table.printstd();
}

fn write_system(state: State) {
    if state.verbose {
        println!("Writing system entry");
    }
    let raw_key = prompt("System key: ");
    let raw_data = prompt("System data: ");

    let rawlink = NewSystem {
        key: &raw_key,
        data: &raw_data,
    };

    let new_system = create_system(&rawlink);
    if state.verbose {
        println!("Saved {}", &new_system.key);
    }
}

fn edit_system(state: State, system_key: String) {
    let mut system_values = read_system(system_key);
    system_values.key = edit_prompt("Edit system key: ", &system_values.key);
    system_values.data = vim_edit(system_values.data);
    let updated_system_result = update_system(&system_values);
    if state.verbose {
        println!("Updated system: {:?}", &updated_system_result);
    }
}

fn delete_a_system(state: State, key: String) {
    if state.verbose {
        println!("Deleting system entry: {}", &key);
    }
    delete_system(&key)
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
        "time": &post_to_export.time,
        "tags": &post_to_export.tags,
        "summary": &post_to_export.summary,
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
