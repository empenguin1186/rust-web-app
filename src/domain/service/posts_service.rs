use crate::domain::repository::posts_repository::PostsRepository;
use crate::models::Post;
use std::error::Error;

pub trait PostsService {
    fn read_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>>;
    fn create_post<'a>(&self, post_title: &'a str, body: &'a str) -> Result<(), Box<dyn Error>>;
    fn update_post(&self, update_id: i32) -> Result<(), Box<dyn Error>>;
    fn delete_post(&self, word: &str) -> Result<(), Box<dyn Error>>;
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

    fn create_post<'a>(&self, post_title: &'a str, body: &'a str) -> Result<(), Box<dyn Error>> {
        self.repository.write_post(post_title, body)
    }

    fn update_post(&self, update_id: i32) -> Result<(), Box<dyn Error>> {
        self.repository.publish_post(update_id)
    }

    fn delete_post(&self, word: &str) -> Result<(), Box<dyn Error>> {
        self.repository.delete_post(word)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::repository::posts_repository::MockPostsRepository;
    use mockall::*;

    #[test]
    fn posts_service_test() {
        let mut mock = MockPostsRepository::new();
        let posts_service = PostsServiceImpl::new(Box::new(&mock));
        let is_published = true;
        let post = vec![Post {
            id: 1,
            title: String::from("title"),
            body: String::from("body"),
            published: true,
        }];
        mock.expect_show_posts()
            .with(predicate::eq(is_published))
            .times(1)
            .returning(|_| Ok(post));
        let result = posts_service.read_posts(is_published);

        match result {
            Ok(n) => assert_eq!(post, n),
            Err(e) => panic!("Unexpected Error Occurred."),
        }
    }
}
