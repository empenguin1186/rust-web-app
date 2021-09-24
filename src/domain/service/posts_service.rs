use crate::domain::repository::posts_repository::PostsRepository;
use crate::models::Post;
use std::error::Error;

pub trait PostsService {
    fn read_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>>;
    fn create_post<'a>(&self, post_title: &'a str, body: &'a str);
    fn update_post(&self, update_id: i32);
    fn delete_post(&self, word: &str);
}

pub struct PostsServiceImpl {
    repository: Box<dyn PostsRepository>,
}

impl PostsServiceImpl {
    pub fn new(repository: Box<dyn PostsRepository>) -> PostsServiceImpl {
        PostsServiceImpl { repository }
    }
}

impl PostsService for PostsServiceImpl {
    fn read_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>> {
        self.repository.show_posts(is_published)
    }

    fn create_post<'a>(&self, post_title: &'a str, body: &'a str) {
        self.repository.write_post(post_title, body)
    }

    fn update_post(&self, update_id: i32) {
        self.repository.publish_post(update_id)
    }

    fn delete_post(&self, word: &str) {
        self.repository.delete_post(word)
    }
}
