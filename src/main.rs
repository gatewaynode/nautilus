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
use std::io::{stdin};
use std::io::prelude::*;
use self::models::{Post, NewPost};
use prettytable::{Table};
use serde_json::json;

// @TODO Separate the interactive mode from the non-interactive mode
// @TODO Add the verbose mode handlers
// @TODO Add post_id checking standard handler (currently just fails if the post_id doesn't exist)
// @TODO Need the import function to handle insert new as well as update existing

fn main() {
    use clap::{load_yaml, App};

    let yaml = load_yaml!("cli.yml");

    let matches = App::from(yaml).get_matches();

    match matches.subcommand() {
        ("list", Some(_clone_matches)) => {
            list_posts()
        }
        ("write", Some(_clone_matches)) => {
            write_post()
        }
        ("edit", Some(_clone_matches)) => {
            // UGLY get's the subcommand arg, unwraps it, parses it as i32, unwraps that or on fail gives it a value of 1
            // Rinse, repeat, soak eyes in bleach
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(1);
            edit_post(this_post)
        }
        ("show", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(1);
            show_post(this_post)
        }
        ("delete", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(1);
            delete_a_post(this_post)
        }
        ("export", Some(_clone_matches)) => {
            let this_post = _clone_matches.value_of("post_id").unwrap().parse::<i32>().unwrap_or(1);
            let export_filename = _clone_matches.value_of("export_filename").unwrap();
            export_post(this_post, export_filename)
        }
        ("import", Some(_clone_matches)) => {
            // println!("NOT IMPLEMENTED!  Import new post from filename {:#?}", _clone_matches.value_of("import_filename"))
            let import_filename = _clone_matches.value_of("import_filename").unwrap();
            import_post(import_filename)
        }
        ("", None) => println!("No subcommand used"), // @TODO add a default action here
        _ => unreachable!(),
    }

}

fn list_posts() {
    let all_posts: Vec<Post> = read_all_posts();
    println!("Displaying {} posts:", all_posts.len());

    let mut table = Table::new();
    table.add_row(row!["ID", "TITLE"]);
    for post in all_posts {
        table.add_row(row![&post.id, &post.title]);
    }
    table.printstd();
}

fn write_post() {

    println!("Writing Post...\nTitle: ");
    let mut raw_title = String::new();
    stdin().read_line(&mut raw_title).unwrap();

    let some_file = NamedTempFile::new();
    let file_path = String::from(some_file.unwrap().path().to_string_lossy());
    println!("Writing Body...");
    let delay = time::Duration::from_millis(750);
    thread::sleep(delay);

    subprocess::Exec::cmd("vim")
        .arg(&file_path)
        .join()
        .unwrap();

    let contents = fs::read_to_string(&file_path)
        .expect("Something went wrong reading the file");

    fs::remove_file(&file_path)
        .expect("Could not cleanup temp file");

    let rawpost = NewPost {
        title: &raw_title,
        body: &contents,
    };

    let post = create_post(&rawpost);
    println!("\nSaved {} with id {}", &rawpost.title, post.id)

}

fn edit_post(post_id: i32) {
    println!("Editing post {}", post_id);

    let some_file = NamedTempFile::new();
    let file_path = String::from(some_file.unwrap().path().to_string_lossy());
    let current_content: Post = read_post(post_id);

    fs::write(&file_path, current_content.body)
        .expect("Something went wrong with writing the temp file");

    subprocess::Exec::cmd("vim")
        .arg(&file_path)
        .join()
        .unwrap();

    let updated_post = Post {
        body: fs::read_to_string(&file_path).unwrap(),
        ..current_content
    };

    let result = update_post(&updated_post).unwrap();
    println!("Update post result: {}", &result)

}

fn show_post(post_id: i32) {
    println!("Showing post {}", post_id);

    let output: Post = read_post(post_id);

    let mut table = Table::new();
    table.add_row(row!["ID", "TITLE", "BODY", "PUBLISHED"]);
    table.add_row(row![output.id, output.title, output.body, output.published]);
    table.printstd();
}

fn delete_a_post(post_id: i32) {
    println!("Deleting post {}", post_id);
    delete_post(post_id)
}

fn export_post(this_post: i32, export_filename: &str) {
    println!("Exporting post ID {} to filename {}", &this_post, &export_filename);
    let post_to_export = read_post(this_post);

    let post_json = json!({
        "id": &post_to_export.id,
        "published": &post_to_export.published,
        "title": &post_to_export.title,
        "body": &post_to_export.body,
    });

    println!("JSON out: {}", post_json.to_string());
    let mut file = fs::File::create(&export_filename)
        .expect("Problem creating export file");
    file.write_all(post_json.to_string().as_bytes())
        .expect("Could not write to export file");

}

fn import_post(import_filename: &str) {
    println!("Importing filename {} as a piece of Post content", &import_filename);
    let file_string = fs::read_to_string(import_filename)
        .expect("Could not open the import filename");
    let imported_post: Post = serde_json::from_str(&file_string).unwrap();

    let result = update_post(&imported_post).unwrap();
    println!("Imported post result: {}", &result)
}
