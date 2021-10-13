use crate::schema::CommentsPE::dsl::{comment_id, path, CommentsPE};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Unsigned};
use diesel::MysqlConnection;

use crate::domain::repository::comments_repository::CommentsRepository;
use crate::models::{CommentPE, NewCommentsPE};
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
            .order(path.asc())
            .load::<CommentPE>(&self.connection);

        match result {
            Ok(n) => return Ok(n),
            Err(e) => return Err(Box::new(e)),
        }
    }

    fn add_comments(&self, id: u64, author: &u64, comment: &str) -> Result<(), Box<dyn Error>> {
        let new_comment = NewCommentsPE { author, comment };

        let insert_result = diesel::insert_into(CommentsPE)
            .values(&new_comment)
            .execute(&self.connection);

        match insert_result {
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }

        let update_result = sql_query(
            "
            UPDATE CommentsPE
              SET path =
                (SELECT x.path FROM (
                  SELECT path FROM CommentsPE WHERE comment_id = ?
                ) AS x) || LAST_INSERT_ID() || '/'
            WHERE comment_id = LAST_INSERT_ID();    
            ",
        )
        .bind::<Unsigned<BigInt>, _>(id)
        .execute(&self.connection);

        match update_result {
            Ok(_) => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }
}
