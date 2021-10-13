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

#[derive(Serialize, Deserialize)]
struct CUDResponse {
    result: Option<String>,
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
    let result = state.post_post(&request.title, &request.body);
    match result {
        Ok(()) => HttpResponse::Ok().json(CUDResponse {
            result: Some(String::from("Ok")),
            error: None,
        }),
        Err(e) => HttpResponse::NotFound().json(CUDResponse {
            result: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[patch("/post")]
async fn patch_post(state: web::Data<PostState>, param: Query<PatchParam>) -> impl Responder {
    let result = state.patch_post(param.id);
    match result {
        Ok(()) => HttpResponse::Ok().json(CUDResponse {
            result: Some(String::from("Ok")),
            error: None,
        }),
        Err(e) => HttpResponse::NotFound().json(CUDResponse {
            result: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[delete("/post")]
async fn delete_post(state: web::Data<PostState>, param: Query<DeleteParam>) -> impl Responder {
    let result = state.delete_post(&param.keyword);
    match result {
        Ok(()) => HttpResponse::Ok().json(CUDResponse {
            result: Some(String::from("Ok")),
            error: None,
        }),
        Err(e) => HttpResponse::NotFound().json(CUDResponse {
            result: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

use crate::domain::model::tree::Tree;
use crate::domain::repository::comments_repository::CommentsRepository;
use crate::infrastructure::repository::comments_repository_impl::CommentsRepositoryImpl;
extern crate serde_json;

fn main() {
    let comments_repository = CommentsRepositoryImpl::new();
    let path = String::from("1/");
    let author = 5;
    // let result = comments_repository.add_comments(1, &author, "hogehogehoge");
    // match result {
    //     _ => {}
    //     Err(e) => println!("error: {}", e),
    // }

    let result = comments_repository.select_comments(&path);
    match result {
        Ok(n) => {
            let tree = Tree::new(&n);
            let json = serde_json::to_string(&tree).unwrap();
            println!("result: {}", json);
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let repository = PostsRepositoryImpl::new();
//     HttpServer::new(move || {
//         let state = PostState::new(Box::new(PostsServiceImpl::new(&repository)));
//         App::new()
//             .service(get_posts)
//             .service(post_post)
//             .service(patch_post)
//             .service(delete_post)
//             .data(state)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

pub struct PostState {
    posts_service: Box<dyn PostsService>,
}

impl PostState {
    async fn new(posts_service: Box<dyn PostsService>) -> PostState {
        PostState { posts_service }
    }

    fn get_posts(&self, is_published: bool) -> Result<Vec<Post>, Box<dyn Error>> {
        self.posts_service.read_posts(is_published)
    }

    fn post_post<'a>(&self, post_title: &'a str, body: &'a str) -> Result<(), Box<dyn Error>> {
        self.posts_service.create_post(post_title, body)
    }

    fn patch_post(&self, update_id: i32) -> Result<(), Box<dyn Error>> {
        self.posts_service.update_post(update_id)
    }

    fn delete_post(&self, word: &str) -> Result<(), Box<dyn Error>> {
        self.posts_service.delete_post(word)
    }
}
