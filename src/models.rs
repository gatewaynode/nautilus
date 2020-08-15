use crate::schema::{nodes, posts, links, system, node_revisions, post_revisions, link_revisions, system_revisions};
use crate::serde_derive::{Serialize, Deserialize};

// @TODO Fix the struct ordering for "post" when we change the name to "article"

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub time: chrono::NaiveDateTime,
    pub tags: String,
    pub summary: String,
    pub version: i32,
    pub updated: chrono::NaiveDateTime,
    pub parent: i32,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct PostRevision {
    pub id: i32,
    pub version: i32,
    pub title: String,
    pub body: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub tags: String,
    pub summary: String,
    pub parent: i32,
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
pub struct Node {
    pub id: i32,
    pub version: i32,
    pub _child_hash: String,
    pub _self_hash: String,
    pub _hash_chain: String,
    pub labels: String,
    pub workflow: String,
    pub permissions: String,
    pub paths_to: String,
    pub paths_from: String,
    pub node_next: String,
    pub node_last: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub child: i32,
    pub child_content_type: String,
}

#[derive(Insertable)]
#[table_name="nodes"]
pub struct NewNode<'a> {
    pub workflow: &'a str,
    pub permissions: &'a str,
}

impl NewNode<'_> {
    pub fn new() -> NewNode<'static> {
        NewNode {
            workflow: "constructing",
            permissions: "pre",
        }
    }
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct NodeRevision {
    pub id: i32,
    pub version: i32,
    pub _child_hash: String,
    pub _self_hash: String,
    pub _hash_chain: String,
    pub labels: String,
    pub workflow: String,
    pub permissions: String,
    pub paths_to: String,
    pub paths_from: String,
    pub node_next: String,
    pub node_last: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub child: i32,
    pub child_content_type: String,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: i32,
    pub text: String,
    pub title: String,
    pub url: String,
    pub tags: String,
    pub time: chrono::NaiveDateTime,
    pub version: i32,
    pub updated: chrono::NaiveDateTime,
    pub parent: i32,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct LinkRevision {
    pub id: i32,
    pub version: i32,
    pub text: String,
    pub title: String,
    pub url: String,
    pub tags: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub parent: i32,
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
    pub version: i32,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name="system_revisions"]
pub struct SystemRevision {
    pub key: String,
    pub version: i32,
    pub data: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="system"]
pub struct NewSystem<'a> {
    pub key: &'a str,
    pub data: &'a str,
}

// Trying to place the relevant enum here
#[derive(Debug)]
pub enum Content {
    PostContent(Post),
    LinkContent(Link),
}

// A data structure representing a full node and content
#[derive(Debug)]
pub struct FullNode {
    node: Node,
    content: Content,
}
