use diesel::prelude::*;
use diesel::MysqlConnection;
use rust_web_app::schema::posts::dsl::{posts, published};

use crate::domain::posts_repository::PostsRepository;
use crate::models::Post;
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
}
