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
extern crate dialoguer;

// use tempfile::NamedTempFile;
use std::fs;
use nautilus::*;
use std::{thread, time};
use std::io::prelude::*;
use self::models::{
    Post,
    NewPost,
    Link,
    NewLink,
    System,
    NewSystem,
    Content,
    Content::LinkContent,
    Content::PostContent
};
use prettytable::{Table};
use serde_json::json;
use simple_prompts::{edit_prompt, prompt};
use vim_edit::{vim_create, vim_edit};
use dialoguer::{theme::ColorfulTheme, Select};
// Testing
use chrono::{NaiveDate, NaiveDateTime};
use chrono::format::ParseError;

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
}

impl State {
    fn new() -> State {
        State {
            verbose: false,
            debug: false,
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
        ("create", Some(_clone_matches)) => {
            create_content(state)
        }
        ("read", Some(_clone_matches)) => {
            read_content()
        }
        ("edit", Some(_clone_matches)) => {
            edit_content(state)
        }
        ("delete", Some(_clone_matches)) => {
            delete_content()
        }
        ("testing", Some(_clone_matches)) => {
            let this_post = Post {
                id: 999,
                title: String::from("Some title"),
                body: String::from("Some body"),
                time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
                tags: String::from("testing"),
                summary: String::from("Some summary"),
                version: 1,
                updated: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
                parent: 1,
            };
            let node = _create_node();
            let content = Content::PostContent(this_post);
            _save_node_content(node, content);
            // let this_link = Link {
            //     id: 999,
            //     text: String::from("Some link"),
            //     title: String::from("Some title"),
            //     url: String::from("http"),
            //     tags: String::from("Testing"),
            //     time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            //     version: 1,
            //     updated: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
            //     parent: 1,
            // };
            // let node = _create_node();
            // let other_content = Content::LinkContent(this_link);
            // _save_node_content(node, other_content);
        }
        ("export", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id")
                .unwrap()
                .parse::<i32>()
                .unwrap_or(0);
            let export_filename = _clone_matches.value_of("export_filename").unwrap();
            export_post(state, this_post, export_filename)
        }
        ("import", Some(_clone_matches)) => {
            let import_filename = _clone_matches.value_of("import_filename").unwrap();
            import_post(state, import_filename)
        }
        ("", None) => println!("No subcommand used"), // @TODO add a default REPL action here
        _ => unreachable!(),
    }
}

// Interactive Functions
fn create_content(state: State) {
    let verbose = state.verbose; // Implicit copy

    let selections = &[
        "Article",
        "Link",
        "System",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the type of content to create: ")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Article" => {
            write_post(state)
        }
        "Link" => {
            write_link(state)
        }
        "System" => {
            write_system(state)
        }
        _ => println!("How did you manage no selection?  ERROR")
    }
    if verbose {
        println!("Creating : {}", selections[selection]);
    }
}

fn read_content() {
    let selections = &[
        "Article",
        "Link",
        "System",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the type of content to create: ")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Article" => {
            let post_id = select_article();
            if post_id != 0 {
                show_post(post_id)
            }
            else {
                println!("Nothing entered, nothing to show.")
            }
        }
        "Link" => {
            let link_id = select_link();
            if link_id != 0 {
                show_link(link_id)
            }
            else {
                println!("Nothing entered, nothing to show.")
            }
        }
        "System" => {
            let key: String = select_system();
            if &key == "" {
                println!("Nothing entered, nothing to do.")
            }
            else {
                show_system(key)
            }
        }
        _ => println!("How did you manage no selection?  ERROR")
    }
}

fn edit_content(state: State) {
    let verbose = state.verbose; // Implicit copy

    let selections = &[
        "Article",
        "Link",
        "System",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the type of content to create: ")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Article" => {
            let article_to_edit: i32 = select_article();
            if article_to_edit != 0 {
                edit_post(state, article_to_edit)
            }
            else {
                println!("Nothing entered to edit, nothing to do.")
            }
        }
        "Link" => {
            let link_to_edit: i32 = select_link();
            if link_to_edit != 0 {
                edit_link(state, link_to_edit)
            }
            else {
                println!("Nothing entered to edit, nothing to do.")
            }
        }
        "System" => {
            let key_to_edit: String = select_system();
            if &key_to_edit == "" {
                println!("Nothing entered to edit, nothing to do.")
            }
            else {
                edit_system(state, key_to_edit)
            }
        }
        _ => println!("How did you manage no selection?  ERROR")
    }
    if verbose {
        println!("Creating : {}", selections[selection]);
    }
}

fn delete_content() {
    let selections = &[
        "Article",
        "Link",
        "System",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the type of content to create: ")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Article" => {
            let post_id = select_article();
            if post_id != 0 {
                prompt("Are you sure?");
                delete_post(post_id)
            }
            else {
                println!("Nothing entered, nothing to delete.")
            }
        }
        "Link" => {
            let link_id = select_link();
            if link_id != 0 {
                prompt("Are you sure?");
                delete_link(link_id)
            }
            else {
                println!("Nothing entered, nothing to delete.")
            }
        }
        "System" => {
            let key: String = select_system();
            if &key == "" {
                println!("Nothing entered, nothing to delete.")
            }
            else {
                prompt("Are you sure?");
                delete_system(&key)
            }
        }
        _ => println!("How did you manage no selection?  ERROR")
    }
}

fn select_article() -> i32 {
    list_posts();
    prompt("Enter an ID number to edit: ")
        .parse::<i32>()
        .unwrap_or(0)
}

fn select_link() -> i32 {
    list_links();
    prompt("Enter an ID number to edit: ")
        .parse::<i32>()
        .unwrap_or(0)
}

fn select_system() -> String {
    list_system();
    prompt("Enter a key to edit: ")
}

// <-- Primary functions -->
fn list_posts() {
    let all_posts: Vec<Post> = read_all_posts();

    let mut table = Table::new();
    table.add_row(row!["ID", "TITLE", "SUMMARY", "TAGS", "TIME"]);
    for post in all_posts {
        table.add_row(row![&post.id, &post.title, &post.summary, &post.tags, &post.time]);
    }
    table.printstd();
}

fn list_links() {
    let all_links: Vec<Link> = read_all_links();

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


fn edit_post(state: State, post_id: i32) {
    if state.verbose {
        println!("Editing post {}", post_id);
    }

    let current_content: Post = read_post(post_id);

    let mut raw_title: String = current_content.title.clone();
    let mut raw_body: String = current_content.body.clone();
    let mut raw_tags: String = current_content.tags.clone();
    let mut raw_summary: String = current_content.summary.clone();

    raw_title = edit_prompt("Edit title: ", &raw_title);
    raw_body = vim_edit(raw_body);
    raw_tags = edit_prompt("Edit tags: ", &raw_tags);
    raw_summary = edit_prompt("Edit summary: ", &raw_summary);

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

fn show_post(post_id: i32) {
    let output: Post = read_post(post_id);
    println!("{:#?}", output)
}

fn show_link(link_id: i32) {
    let output: Link = read_link(link_id);
    println!("{:#?}", output)
}

fn show_system(key: String) {
    let output: System = read_system(key);
    println!("{:#?}", output)
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

fn edit_link(state: State, link_id: i32) {
    if state.verbose {
        println!("Editing link {}", link_id);
    }

    let current_content: Link = read_link(link_id);

    let mut raw_text: String = current_content.text.clone();
    let mut raw_title: String = current_content.title.clone();
    let mut raw_url: String = current_content.url.clone();
    let mut raw_tags: String = current_content.tags.clone();

    raw_text = edit_prompt("Edit text: ", &raw_text);
    raw_title = edit_prompt("Edit title: ", &raw_title);
    raw_url = edit_prompt("Edit url: ", &raw_url);
    raw_tags = edit_prompt("Edit tags: ", &raw_tags);

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

fn list_system() {
    let all_system: Vec<System> = read_all_system();

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

fn export_post(state: State, this_post: i32, export_filename: &str) {
    if state.verbose {
        println!("Exporting post ID {} to filename {}", &this_post, &export_filename);
    }
    let post_to_export = read_post(this_post);

    let post_json = json!({
        "id": &post_to_export.id,
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
