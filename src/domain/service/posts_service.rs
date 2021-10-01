use crate::domain::repository::posts_repository::PostsRepository;
use crate::models::Post;
use std::error::Error;

pub trait PostsService {
    fn read_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>>;
    fn create_post(&self, post_title: &str, body: &str) -> Result<(), Box<dyn Error>>;
    fn update_post(&self, update_id: i32) -> Result<(), Box<dyn Error>>;
    fn delete_post(&self, word: &str) -> Result<(), Box<dyn Error>>;
}

pub struct PostsServiceImpl<'a, T> {
    repository: &'a T,
}

impl<'a, T: PostsRepository> PostsServiceImpl<'a, T> {
    pub fn new(repository: &'a T) -> PostsServiceImpl<T> {
        PostsServiceImpl { repository }
    }
}

impl<'a, T: PostsRepository> PostsService for PostsServiceImpl<'a, T> {
    fn read_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>> {
        self.repository.show_posts(is_published)
    }

    fn create_post(&self, post_title: &str, body: &str) -> Result<(), Box<dyn Error>> {
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
        let is_published = true;
        let post = vec![Post {
            id: 1,
            title: String::from("title"),
            body: String::from("body"),
            published: true,
        }];
        let returned = post.clone();
        mock.expect_show_posts()
            .with(predicate::eq(is_published))
            .times(1)
            .returning(move |_| Ok(returned.clone()));

        let posts_service = PostsServiceImpl::new(&mock);
        let result = posts_service.read_posts(is_published);

        match result {
            Ok(n) => assert_eq!(post, n),
            Err(e) => panic!("Unexpected Error Occurred."),
        }
    }
}
