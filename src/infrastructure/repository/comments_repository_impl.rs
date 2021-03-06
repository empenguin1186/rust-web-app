use std::error::Error;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Unsigned};
use diesel::MysqlConnection;

use crate::domain::repository::comments_repository::CommentsRepository;
use crate::models::{CommentPE, NewCommentsPE};
use crate::schema::CommentsPE::dsl::{path, CommentsPE};
use diesel::r2d2::{PooledConnection, ConnectionManager};

pub struct CommentsRepositoryImpl {
    connection: PooledConnection<ConnectionManager<MysqlConnection>>,
}

impl CommentsRepositoryImpl {
    pub fn new(connection: PooledConnection<ConnectionManager<MysqlConnection>>) -> CommentsRepositoryImpl {
        CommentsRepositoryImpl { connection }
    }
}

impl CommentsRepository for CommentsRepositoryImpl {
    fn get_path(&self, id: u64) -> Result<Option<String>, Box<dyn Error>> {
        let result = CommentsPE
            .find(id)
            .select(path)
            .first::<Option<String>>(&self.connection);

        return match result {
            Ok(n) => Ok(n),
            Err(e) => Err(Box::new(e)),
        };
    }

    fn select_comments(&self, target_path: &str) -> Result<Vec<CommentPE>, Box<dyn Error>> {
        let pattern = format!("{}%", target_path);

        let result = CommentsPE
            .filter(path.like(pattern))
            .order(path.asc())
            .load::<CommentPE>(&self.connection);

        return match result {
            Ok(n) => Ok(n),
            Err(e) => Err(Box::new(e)),
        };
    }

    fn add_comments(&self, id: u64, author: &u64, comment: &str) -> Result<(), Box<dyn Error>> {
        let transaction_result = self.connection.transaction(|| {
            let new_comment = NewCommentsPE { author, comment };
            let insert_result = diesel::insert_into(CommentsPE)
                .values(&new_comment)
                .execute(&self.connection);

            if let Err(_) = insert_result {
                return Err(DieselError::RollbackTransaction);
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

            return match update_result {
                Ok(_) => Ok(()),
                Err(_) => Err(DieselError::RollbackTransaction),
            };
        });

        return match transaction_result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        };
    }
}
