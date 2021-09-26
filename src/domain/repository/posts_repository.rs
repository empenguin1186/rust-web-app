use crate::models::Post;
use mockall::predicate::*;
use mockall::*;
use std::error::Error;

#[automock]
pub trait PostsRepository {
    fn show_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>>;
    fn write_post<'a>(&self, post_title: &'a str, body: &'a str) -> Result<(), Box<dyn Error>>;
    fn publish_post(&self, update_id: i32) -> Result<(), Box<dyn Error>>;
    fn delete_post(&self, word: &str) -> Result<(), Box<dyn Error>>;
}
