#[macro_use]
extern crate diesel;
extern crate dotenv;
mod domain;
mod infrastructure;
pub mod models;
pub mod schema;
use crate::domain::service::posts_service::{PostsService, PostsServiceImpl};
use crate::models::Post;
use actix_web::web::Query;
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use infrastructure::repository::posts_repository_impl::PostsRepositoryImpl;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize)]
struct PostParam {
    is_published: bool,
}

#[derive(Deserialize)]
struct PatchParam {
    id: i32,
}

#[derive(Deserialize)]
struct DeleteParam {
    keyword: String,
}

#[derive(Deserialize)]
struct RequestPost {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct GetPostResponse {
    results: Option<Vec<Post>>,
    error: Option<String>,
}

#[get("/post")]
async fn get_posts(state: web::Data<PostState>, param: Query<PostParam>) -> impl Responder {
    let is_published = param.is_published;
    let result = state.get_posts(is_published);
    match result {
        Ok(n) => HttpResponse::Ok().json(GetPostResponse {
            results: Some(n),
            error: None,
        }),
        Err(e) => HttpResponse::NotFound().json(GetPostResponse {
            results: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[post("/post")]
async fn post_post(state: web::Data<PostState>, request: web::Json<RequestPost>) -> impl Responder {
    state.post_post(&request.title, &request.body);
    format!("Registered {}!", request.title)
}

#[patch("/post")]
async fn patch_post(state: web::Data<PostState>, param: Query<PatchParam>) -> impl Responder {
    state.patch_post(param.id);
    format!("Update Succeeded! id: {}!", param.id)
}

#[delete("/post")]
async fn delete_post(state: web::Data<PostState>, param: Query<DeleteParam>) -> impl Responder {
    state.delete_post(&param.keyword);
    format!("Delete Succeeded! keyword: {}!", param.keyword)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_posts)
            .service(post_post)
            .service(patch_post)
            .service(delete_post)
            .data(PostState::new(Box::new(PostsServiceImpl::new(Box::new(
                PostsRepositoryImpl::new(),
            )))))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub struct PostState {
    posts_service: Box<dyn PostsService>,
}

impl PostState {
    pub fn new(posts_service: Box<dyn PostsService>) -> PostState {
        PostState { posts_service }
    }

    fn get_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>> {
        self.posts_service.read_posts(is_published)
    }

    fn post_post<'a>(&self, post_title: &'a str, body: &'a str) {
        self.posts_service.create_post(post_title, body)
    }

    fn patch_post(&self, update_id: i32) {
        self.posts_service.update_post(update_id)
    }

    fn delete_post(&self, word: &str) {
        self.posts_service.delete_post(word)
    }
}
