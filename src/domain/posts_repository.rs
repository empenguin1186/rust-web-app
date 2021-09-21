use crate::models::Post;

pub trait PostsRepository {
    fn show_posts(&self, is_published: bool) -> Vec<Post>;
}
