extern crate diesel;
extern crate rust_web_app;

use self::diesel::prelude::*;
use self::rust_web_app::*;

use std::env::args;

fn main() {
    use rust_web_app::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
