use crate::models::CommentPE;
use mockall::predicate::*;
use mockall::*;
use std::error::Error;

#[automock]
pub trait CommentsRepository {
    fn select_comments(&self, target_path: &String) -> Result<Vec<CommentPE>, Box<dyn Error>>;
    fn add_comments(&self, id: u64, author: &u64, comment: &str) -> Result<(), Box<dyn Error>>;
}
