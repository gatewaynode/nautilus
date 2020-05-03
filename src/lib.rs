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
use self::models::{Post, Link, NewPost, NewLink, System, NewSystem};

/// Create a database connection.  Does not use pools, so this is not suitable for prod connections.
///
/// ```
/// use nautilus::*;
///
/// fn connection_test() {
///   let connection = establish_connection();
/// }
/// ```
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect("Error connecting to database.")
}

/// Enter a NewPost struct into the database (tracks closely to Post without the auto fields).
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{NewPost};
///
/// fn post_something() {
///   let thingy = NewPost {
///     title: "Something",
///     body: "Something",
///     summary: "Something",
///     tags: "This, That",
///   };
///
///   let newpost = create_post(&thingy);
///   println!("{:?}", newpost)
///
/// }
/// ```
pub fn create_post(content: &NewPost) -> Post {
    use schema::posts;

    let connection = establish_connection();

    diesel::insert_into(posts::table)
        .values(content)
        .get_result(&connection)
        .expect("Error saving new post")
}


/// Update an existing post
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{Post};
///
/// // Note the next 3 lines are just needed to pass the doctest or if your are manually setting time
/// extern crate chrono;
/// use chrono::{NaiveDate, NaiveDateTime};
/// use chrono::format::ParseError;
///
/// fn update_some_post() {
///   let thingy = Post {
///     id: 1,
///     time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
///     published: false,
///     title: String::from("Somethin else"),
///     body: String::from("Something"),
///     summary: String::from("Something else"),
///     tags: String::from("This, That"),
///     comment_url: String::from("https://www.google.com"),
///   };
///
///   let newpost = update_post(&thingy);
///   println!("{:?}", newpost);
///
/// }
/// ```
pub fn update_post(content: &Post) -> QueryResult<usize>{
    let connection = establish_connection();

    diesel::update(content).set(content).execute(&connection)

}

/// Read a post by post id
///
/// ```
/// use nautilus::*;
///
/// fn update_some_post() {
///   let some_post = read_post(1);
///   println!("{}", some_post.body)
/// }
/// ```
pub fn read_post(post_id: i32) -> Post{
    use schema::posts::dsl::*;

    let connection = establish_connection();

    posts.filter(id.eq(post_id))
        .limit(1)
        .get_result::<Post>(&connection)
        .expect("Error loading post by that ID")
}

/// Read all the posts into a Vec that can be iterated through
///
/// ```
/// use nautilus::*;
///
/// fn update_some_post() {
///   let all_posts = read_all_posts();
///   for post in all_posts {
///     println!("{}", post.title);
///   }
/// }
/// ```
pub fn read_all_posts() -> Vec<Post> {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    posts
        .order(id.desc())
        .load::<Post>(&connection)
        .expect("Error loading posts")
}


/// Read all the posts with a given tag pattern up to the given limit.
///
/// ```
/// use nautilus::*;
///
/// fn update_some_post() {
///   let all_posts = read_posts_by_filter_limit(String::from("Groovy"), 5);
///   for post in all_posts {
///     println!("{}", post.title);
///   }
/// }
/// ```
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

/// Currently not in use, but when it is it simply sets the published field to ``true``
pub fn publish_post(post_id: i32) -> Post {
    use schema::posts::dsl::{posts, published};

    let connection = establish_connection();

    diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect("Unable to find post number")
}

/// Delete a post by post id
///
/// ```
/// use nautilus::*;
///
/// fn delete_post(post_id: i32) {
///   delete_post(post_id)
/// }
/// ```
pub fn delete_post(post_id: i32) {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&connection)
        .expect("Error deleting post");
}

pub fn read_link(link_id: i32) -> Link{
    use schema::links::dsl::*;

    let connection = establish_connection();

    links.filter(id.eq(link_id))
        .limit(1)
        .get_result::<Link>(&connection)
        .expect("Error loading post by that ID")
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
        .order(id.desc())
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

pub fn update_link(content: &Link) -> QueryResult<usize> {
    let connection = establish_connection();

    diesel::update(content).set(content).execute(&connection)

}

pub fn read_all_system() -> Vec<System> {
    use schema::system::dsl::*;
    let connection = establish_connection();

    system
        .load::<System>(&connection)
        .expect("Error loading system")
}

pub fn read_system(system_key: String) -> System {
    use schema::system::dsl::*;

    let connection = establish_connection();

    system.filter(key.like(&system_key))
        .limit(1)
        .get_result::<System>(&connection)
        .expect("System key error")
}

pub fn create_system(content: &NewSystem) -> System {
    use schema::system;

    let connection = establish_connection();
    diesel::insert_into(system::table)
        .values(content)
        .get_result(&connection)
        .expect("Error saving new link")
}

pub fn update_system(content: &System) -> QueryResult<usize> {
    use schema::system;
    let connection = establish_connection();

    diesel::update(system::table).set(content).execute(&connection)
}

pub fn delete_system(system_key: &str) {
    use schema::system::dsl::*;
    let connection = establish_connection();

    diesel::delete(system.filter(key.like(system_key)))
        .execute(&connection)
        .expect("Error deleting system");
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
