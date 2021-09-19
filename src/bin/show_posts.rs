extern crate diesel;
extern crate rust_web_app;

use self::diesel::prelude::*;
use self::models::*;
use self::rust_web_app::*;

fn main() {
    use rust_web_app::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
