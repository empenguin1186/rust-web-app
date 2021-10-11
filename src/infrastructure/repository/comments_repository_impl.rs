use crate::schema::CommentsPE::dsl::{comment_id, path, CommentsPE};
use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::domain::repository::comments_repository::CommentsRepository;
use crate::models::CommentPE;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct CommentsRepositoryImpl {
    pub connection: MysqlConnection,
}

impl CommentsRepositoryImpl {
    pub fn new() -> CommentsRepositoryImpl {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let connection = MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        CommentsRepositoryImpl { connection }
    }
}

impl CommentsRepository for CommentsRepositoryImpl {
    fn select_comments(&self, target_path: &String) -> Result<Vec<CommentPE>, Box<dyn Error>> {
        let pattern = format!("{}%", target_path);

        let result = CommentsPE
            .filter(path.like(pattern))
            .load::<CommentPE>(&self.connection);

        match result {
            Ok(n) => return Ok(n),
            Err(e) => return Err(Box::new(e)),
        }
    }

    fn add_comments(&self, comment: CommentPE) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
