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
extern crate dirs;
use std::path::Path;
use std::env;
use self::models::{Post, Link, NewPost, NewLink, System, NewSystem};

/// Create a database connection.
///
/// Order of operations here is: 1, environment variables first; 2, local .env file from the
/// execution directory; 3, a .env file in ~/config/N4/.env
///
/// Does not use pools, so this is not suitable for prod connections.
///
/// ```
/// use nautilus::*;
///
/// fn connection_test() {
///   let connection = establish_connection();
/// }
/// ```
pub fn establish_connection() -> PgConnection {
    match dotenv() {
        Ok(_) => {}
        Err(_) => {
            let config_dir = format!("{}/N4/.env", dirs::config_dir()
                .unwrap()
                .display()
                .to_string()
            );
            println!("{}", config_dir);  // @TODO remove this at some point in time.
            match dotenv::from_path(Path::new(&config_dir)) {
                Ok(_) => {}
                Err(e) => {
                    println!("Environment variables not loaded, error.   N4 expects a environment variable of N4_DATABASE_URL or a .dotenv file in the current dir or the local config dir of ~/.config/N4/.env");
                    panic!(e);
                }
            }
        }
    }

    let database_url = env::var("N4_DATABASE_URL")
        .expect("DATABASE_URL must be set!  N4 expects a environment variable of N4_DATABASE_URL or a .dotenv file in the current dir or the local config dir of ~/.config/N4/.env");
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
/// fn read_some_post() {
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
/// fn read_all_the_posts() {
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
        .order(id.asc())
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

/// Read a limited number of posts into a Vec that can be iterated through
///
/// ```
/// use nautilus::*;
///
/// fn read_all_the_posts() {
///   let all_posts = read_all_posts();
///   for post in all_posts {
///     println!("{}", post.title);
///   }
/// }
/// ```
pub fn read_some_posts(limit_value: i64) -> Vec<Post> {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    posts
        .order(id.desc())
        .limit(limit_value)
        .load::<Post>(&connection)
        .expect("Error loading posts")
}


/// Read all the posts with a given tag pattern up to the given limit.
///
/// ```
/// use nautilus::*;
///
/// fn read_some_posts() {
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
/// fn delete_a_post() {
///   delete_post(1)
/// }
/// ```
pub fn delete_post(post_id: i32) {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&connection)
        .expect("Error deleting post");
}

/// Read a link by link id
///
/// ```
/// use nautilus::*;
///
/// fn read_a_link() {
///   let some_link = read_link(1);
///   println!("{}", some_link.text)
/// }
/// ```
pub fn read_link(link_id: i32) -> Link{
    use schema::links::dsl::*;

    let connection = establish_connection();

    links.filter(id.eq(link_id))
        .limit(1)
        .get_result::<Link>(&connection)
        .expect("Error loading post by that ID")
}

/// Read all links
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{Link};
///
/// fn read_all_the_links() {
///   let all_links: Vec<Link> = read_all_links();
///   for link in all_links {
///     println!("{}", link.text);
///   }
/// }
/// ```
pub fn read_all_links() -> Vec<Link> {
    use schema::links::dsl::*;

    let connection = establish_connection();

    links
        .order(id.asc())
        .load::<Link>(&connection)
        .expect("Error loading links")
}

/// Read all the posts with a given tag pattern up to the given limit.
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{Link};
///
/// fn read_some_links() {
///   let some_links: Vec<Link> = read_links_by_filter_limit(String::from("Groovy"), 5);
///   for link in some_links {
///     println!("{}", link.text);
///   }
/// }
/// ```
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

/// Enter a NewLink struct into the database (tracks closely to Link without the auto fields).
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{NewLink};
///
/// fn link_something() {
///   let thingy = NewLink {
///     text: "Something",
///     title: "Something",
///     url: "https://duckduckgo.com/",
///     tags: "This, that",
///   };
///
///   let newlink = create_link(&thingy);
///   println!("{:?}", newlink)
///
/// }
/// ```
pub fn create_link(content: &NewLink) -> Link {
    use schema::links;

    let connection = establish_connection();

    diesel::insert_into(links::table)
        .values(content)
        .get_result(&connection)
        .expect("Error saving new link")
}

/// Update an existing link
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{Link};
///
/// // Note the next 3 lines are just needed to pass the doctest or if your are manually setting time
/// extern crate chrono;
/// use chrono::{NaiveDate, NaiveDateTime};
/// use chrono::format::ParseError;
///
/// fn update_some_post() {
///   let thingy = Link {
///     id: 1,
///     published: false,
///     time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
///     text: String::from("Somethin else"),
///     title: String::from("Something"),
///     url: String::from("Something else"),
///     tags: String::from("This, That"),
///   };
///
///   let uplink = update_link(&thingy);
///   println!("{:?}", uplink);
///
/// }
/// ```
pub fn update_link(content: &Link) -> QueryResult<usize> {
    let connection = establish_connection();

    diesel::update(content).set(content).execute(&connection)

}

/// Delete a link by link id
///
/// ```
/// use nautilus::*;
///
/// fn delete_a_post() {
///   delete_post(1)
/// }
/// ```
pub fn delete_link(link_id: i32) {
    use schema::links::dsl::*;

    let connection = establish_connection();

    diesel::delete(links.filter(id.eq(link_id)))
        .execute(&connection)
        .expect("Error deleting link");
}

/// Read all system entries
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{System};
///
/// fn read_all_system_entries() {
///   let all_systems: Vec<System> = read_all_system();
///   for system in all_systems {
///     println!("{}", system.key);
///   }
/// }
/// ```
pub fn read_all_system() -> Vec<System> {
    use schema::system::dsl::*;
    let connection = establish_connection();

    system
        .load::<System>(&connection)
        .expect("Error loading system")
}

/// Read a system entries by system key value
///
/// ```
/// use nautilus::*;
///
/// fn read_a_system_entry() {
///   let a_system = read_system(String::from("routes"));
///   println!("{:?}", a_system)
/// }
/// ```
pub fn read_system(system_key: String) -> System {
    use schema::system::dsl::*;

    let connection = establish_connection();

    system.filter(key.like(&system_key))
        .limit(1)
        .get_result::<System>(&connection)
        .expect("System key error")
}

/// Enter a NewSystem struct into the database (tracks closely to System without the auto fields).
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{NewSystem};
///
/// fn system_something() {
///   let thingy = NewSystem {
///     key: "This",
///     data: "That"
///   };
///
///   let newsys = create_system(&thingy);
///   println!("{:?}", newsys)
///
/// }
/// ```
pub fn create_system(content: &NewSystem) -> System {
    use schema::system;

    let connection = establish_connection();
    diesel::insert_into(system::table)
        .values(content)
        .get_result(&connection)
        .expect("Error saving new link")
}

/// Update an existing system
///
/// ```
/// use nautilus::*;
/// use nautilus::models::{System};
///
/// // Note the next 3 lines are just needed to pass the doctest or if your are manually setting time
/// extern crate chrono;
/// use chrono::{NaiveDate, NaiveDateTime};
/// use chrono::format::ParseError;
///
/// fn update_some_system() {
///   let thingy = System {
///     key: String::from("routes"),
///     data: String::from("/post/"),
///     time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
///   };
///
///   let upsys = update_system(&thingy);
///   println!("{:?}", upsys);
///
/// }
/// ```
pub fn update_system(content: &System) -> QueryResult<usize> {
    use schema::system;
    let connection = establish_connection();

    diesel::update(system::table).set(content).execute(&connection)
}

/// Delete a link by link id
///
/// ```
/// use nautilus::*;
///
/// fn delete_a_system() {
///   delete_system("routes")
/// }
/// ```
pub fn delete_system(system_key: &str) {
    use schema::system::dsl::*;
    let connection = establish_connection();

    diesel::delete(system.filter(key.like(system_key)))
        .execute(&connection)
        .expect("Error deleting system");
}

/// Not used yet
pub fn publish_link(link_id: i32) -> Link {
    use schema::links::dsl::{links, published};

    let connection = establish_connection();

    diesel::update(links.find(link_id))
        .set(published.eq(true))
        .get_result::<Link>(&connection)
        .expect("Unable to find post number")
}
