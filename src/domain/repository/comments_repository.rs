use std::error::Error;

use mockall::*;
use mockall::predicate::*;

use crate::models::CommentPE;

#[automock]
pub trait CommentsRepository {
    fn get_path(&self, id: u64) -> Result<Option<String>, Box<dyn Error>>;
    fn select_comments(&self, target_path: &str) -> Result<Vec<CommentPE>, Box<dyn Error>>;
    fn add_comments(&self, id: u64, author: &u64, comment: &str) -> Result<(), Box<dyn Error>>;
}
