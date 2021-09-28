use crate::schema::posts::dsl::{posts, published, title};
use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::domain::repository::posts_repository::PostsRepository;
use crate::models::{NewPost, Post};
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct PostsRepositoryImpl {
    pub connection: MysqlConnection,
}

impl PostsRepositoryImpl {
    pub fn new() -> PostsRepositoryImpl {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let connection = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        PostsRepositoryImpl { connection }
    }
}

impl PostsRepository for PostsRepositoryImpl {
    fn show_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>> {
        let result = posts
            .filter(published.eq(is_published))
            .limit(5)
            .load::<Post>(&self.connection);

        match result {
            Ok(n) => return Ok(n),
            Err(e) => return Err(Box::new(e)),
        }
    }

    fn write_post(&self, post_title: &str, body: &str) -> Result<(), Box<dyn Error>> {
        let new_post = NewPost {
            title: post_title,
            body,
        };

        let result = diesel::insert_into(posts)
            .values(&new_post)
            .execute(&self.connection);

        match result {
            Ok(_n) => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }

    fn publish_post(&self, update_id: i32) -> Result<(), Box<dyn Error>> {
        let result = diesel::update(posts.find(update_id))
            .set(published.eq(true))
            .execute(&self.connection);

        match result {
            Ok(_n) => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }

    fn delete_post(&self, word: &str) -> Result<(), Box<dyn Error>> {
        let pattern = format!("%{}%", word);

        let result = diesel::delete(posts.filter(title.like(pattern))).execute(&self.connection);

        match result {
            Ok(_n) => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }
}
