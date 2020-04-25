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
use self::models::{Post, NewPost};

// #[derive(Deserialize, Debug)]
// pub struct PostList { vec!(Post); }

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn read_all_posts() -> Vec<Post>{
    use schema::posts::dsl::*;

    let connection = establish_connection();

    let results = posts
        .order(id.desc())
        .load::<Post>(&connection)
        .expect("Error loading posts");

    return results
}

pub fn create_post<'a>(content: &NewPost) -> Post {
    use schema::posts;

    let connection = establish_connection();

    // Just used the passed struct???
    let new_post = NewPost {
        title: content.title,
        body: content.body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
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

    let result = posts.filter(id.eq(post_id))
        .limit(1)
        .get_result::<Post>(&connection)
        .expect("Error loading post by that ID");

    return result
}



pub fn publish_post(post_id: i32) {
    use schema::posts::dsl::{posts, published};

    let connection = establish_connection();

    let post = diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post number {}", post_id));
    println!("Published post {}", post.title)
}

pub fn delete_post(post_id: i32) {
    use schema::posts::dsl::*;

    let connection = establish_connection();

    let _deleted = diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&connection)
        .expect("Error deleting post");
    println!("Deleted post number {}", post_id)
}

// @TODO Not immediately necessary, but needs to be done
pub fn does_id_exist(post_id: i32) -> bool {
    use schema::posts::dsl::*;
    let connection = establish_connection();

    let result = posts.filter(id.eq(&post_id))
        .first::<Post>(&connection);

    println!("{:#?}", result);
    return false
}

#[cfg(test)]
mod tests {
    fn it_works() {
        assert_eq!(2 + 2 = 4);
    }
}
