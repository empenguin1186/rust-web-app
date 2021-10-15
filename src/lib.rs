#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use self::models::{NewPost, Post};

pub mod models;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connectiong to {}", database_url))
}

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) -> Post {
    use self::schema::posts::dsl::{id, posts};

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    // id の降順でソート(order 句)し、上位1レコードを取得(first 句)する
    posts.order(id.desc()).first(conn).unwrap()
}
