use crate::schema::posts::dsl::{posts, published, title};
use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::domain::posts_repository::PostsRepository;
use crate::models::{NewPost, Post};
use dotenv::dotenv;
use std::env;

pub struct PostsRepositoryImpl {
    pub connection: MysqlConnection,
}

impl PostsRepositoryImpl {
    pub fn new() -> PostsRepositoryImpl {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let connection = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connectiong to {}", database_url));

        PostsRepositoryImpl { connection }
    }
}

impl PostsRepository for PostsRepositoryImpl {
    fn show_posts(&self, is_published: bool) -> Vec<Post> {
        let results = posts
            .filter(published.eq(is_published))
            .limit(5)
            .load::<Post>(&self.connection)
            .expect("Error loading posts");

        results
    }

    fn write_post<'a>(&self, post_title: &'a str, body: &'a str) {
        let new_post = NewPost {
            title: post_title,
            body: body,
        };

        diesel::insert_into(posts)
            .values(&new_post)
            .execute(&self.connection)
            .expect("Error saving new post");
    }

    fn publish_post(&self, update_id: i32) {
        diesel::update(posts.find(update_id))
            .set(published.eq(true))
            .execute(&self.connection)
            .expect("Error updating specified post");
    }

    fn delete_post(&self, word: &str) {
        let pattern = format!("%{}%", word);

        diesel::delete(posts.filter(title.like(pattern)))
            .execute(&self.connection)
            .expect("Error deleting posts");
    }
}