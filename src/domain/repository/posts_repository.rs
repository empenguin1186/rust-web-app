use crate::models::Post;
use std::error::Error;

pub trait PostsRepository {
    fn show_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>>;
    fn write_post<'a>(&self, post_title: &'a str, body: &'a str);
    fn publish_post(&self, update_id: i32);
    fn delete_post(&self, word: &str);
}
