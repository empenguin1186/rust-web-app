extern crate diesel;
extern crate rust_web_app;

use self::diesel::prelude::*;
use self::rust_web_app::*;

use self::models::Post;
use std::env::args;

fn main() {
    use rust_web_app::schema::posts::dsl::{id, posts, published};

    let update_id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("invalid ID");
    let connection = establish_connection();

    diesel::update(posts.find(update_id))
        .set(published.eq(true))
        .execute(&connection)
        .expect("Error updating specified post");

    let results = posts
        .filter(id.eq(update_id))
        .limit(1)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    for post in results {
        println!("Published post {}", post.title);
    }
}
