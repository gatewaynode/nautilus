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
    pub parent: String,
    pub version: i32,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct PostRevision {
    pub id: i32,
    pub version: i32,
    pub parent: String,
    pub title: String,
    pub body: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
    pub tags: String,
    pub summary: String,
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
    pub child: String,
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
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct NodeRevision {
    pub id: i32,
    pub version: i32,
    pub child: String,
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
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: i32,
    pub text: String,
    pub title: String,
    pub url: String,
    pub tags: String,
    pub time: chrono::NaiveDateTime,
    pub parent: String,
    pub version: i32,
    pub updated: chrono::NaiveDateTime,
}

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct LinkRevision {
    pub id: i32,
    pub version: i32,
    pub parent: String,
    pub text: String,
    pub title: String,
    pub url: String,
    pub tags: String,
    pub time: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,
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

// // Node Notes
// #[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
// pub struct Node {
//     pub id: i32,
//     pub version: i32, // Version
//     pub child: String, // Node content, one to one.  Need to store a serialized value not an int
//     pub _child_hash: String, // A hash of the child row //Blake2b
//     pub _self_hash: String,  // A hash of the node row without other hashes //Blake2b
//     pub _hash_chain: String, // A hash of the other two hashes and the last revision _hash_chain if it exists //Blake2b
//     pub labels: String, // Metadata
//     pub workflow: String,
//     pub permissions: String,
//     pub paths_to: String,  // Serialized Vec?
//     pub paths_from: String,
//     pub node_next: i32,
//     pub node_last: i32,
//     pub time: chrono::NaiveDateTime,
//     pub updated: chrono::NiaveDateTime,
// }
