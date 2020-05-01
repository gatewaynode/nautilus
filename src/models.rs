use crate::schema::{posts, links, system};
use crate::serde_derive::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub published: bool,
    pub title: String,
    pub body: String,
    pub time: chrono::NaiveDateTime,
    pub tags: String,
    pub summary: String,
    pub comment_url: String,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub tags: &'a str,
    pub summary: &'a str,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: i32,
    pub published: bool,
    pub text: String,
    pub title: String,
    pub url: String,
    pub tags: String,
    pub time: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="links"]
pub struct NewLink<'a> {
    pub text: &'a str,
    pub title: &'a str,
    pub url: &'a str,
    pub tags: &'a str,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name="system"]
pub struct System {
    pub key: String,
    pub data: String,
    pub time: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="system"]
pub struct NewSystem<'a> {
    pub key: &'a str,
    pub data: &'a str,
}
