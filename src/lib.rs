pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Post, Link, NewPost, NewLink};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect("Error connecting to database.")
}


pub fn create_post(content: &NewPost) -> Post {
    use schema::posts;

    let connection = establish_connection();

    // Just used the passed struct???
    // let new_post = NewPost {
    //     title: content.title,
    //     body: content.body,
    // };

    diesel::insert_into(posts::table)
        .values(content)
        .get_result(&connection)
        .expect("Error saving new post")
}

pub fn update_post(content: &Post) -> QueryResult<usize>{
    let connection = establish_connection();

    diesel::update(content).set(content).execute(&connection)

}

pub fn read_post(post_id: i32) -> Post{
    use schema::posts::dsl::*;

    let connection = establish_connection();

    posts.filter(id.eq(post_id))
        .limit(1)
        .get_result::<Post>(&connection)
        .expect("Error loading post by that ID")
}

pub fn read_all_posts() -> Vec<Post> {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    posts
        .order(id.desc())
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

pub fn read_posts_by_filter_limit(filter_value: String, limit_value: i64) -> Vec<Post> {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    let real_filter_value = format!("%%{}%%", filter_value);

    posts
        .filter(tags.like(&real_filter_value))
        .limit(limit_value)
        .order(id.desc())
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

pub fn publish_post(post_id: i32) -> Post {
    use schema::posts::dsl::{posts, published};

    let connection = establish_connection();

    diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect("Unable to find post number")
}

pub fn delete_post(post_id: i32) {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&connection)
        .expect("Error deleting post");
}

pub fn read_all_links() -> Vec<Link> {
    use schema::links::dsl::*;

    let connection = establish_connection();

    links
        .order(id.desc())
        .load::<Link>(&connection)
        .expect("Error loading links")
}

pub fn read_links_by_filter_limit(filter_value: String, limit_value: i64) -> Vec<Link> {
    use schema::links::dsl::*;

    let connection = establish_connection();

    let real_filter_value = format!("%%{}%%", filter_value);

    links
        .filter(tags.like(&real_filter_value))
        .limit(limit_value)
        .order(id.asc())
        .load::<Link>(&connection)
        .expect("Error loading links")
}

pub fn create_link(content: &NewLink) -> Link {
    use schema::links;

    let connection = establish_connection();

    diesel::insert_into(links::table)
        .values(content)
        .get_result(&connection)
        .expect("Error saving new link")
}

pub fn delete_link(link_id: i32) {
    use schema::links::dsl::*;

    let connection = establish_connection();

    diesel::delete(links.filter(id.eq(link_id)))
        .execute(&connection)
        .expect("Error deleting link");
}

pub fn publish_link(link_id: i32) -> Link {
    use schema::links::dsl::{links, published};

    let connection = establish_connection();

    diesel::update(links.find(link_id))
        .set(published.eq(true))
        .get_result::<Link>(&connection)
        .expect("Unable to find post number")
}

// @TODO Not immediately necessary, but needs to be done
// pub fn does_id_exist(post_id: i32) -> bool {
//     use schema::posts::dsl::*;
//     let connection = establish_connection();
//
//     posts.filter(id.eq(&post_id))
//         .first::<Post>(&connection)
// }
