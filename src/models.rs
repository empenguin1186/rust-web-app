use super::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CommentPE {
    pub id: u64,
    pub path: Option<String>,
    pub author: u64,
    pub comment: String,
}
