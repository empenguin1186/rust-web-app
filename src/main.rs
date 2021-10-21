#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;

use std::env;
use std::error::Error;

use actix_web::{App, delete, get, HttpResponse, HttpServer, patch, post, Responder, web};
use actix_web::web::Query;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use serde::{Deserialize, Serialize};

use infrastructure::repository::posts_repository_impl::PostsRepositoryImpl;

use crate::domain::model::tree::Tree;
use crate::domain::repository::comments_repository::CommentsRepository;
use crate::domain::service::posts_service::{PostsService, PostsServiceImpl};
use crate::infrastructure::repository::comments_repository_impl::CommentsRepositoryImpl;
use crate::models::Post;

mod domain;
mod infrastructure;
pub mod models;
pub mod schema;

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

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/comment/{comment_id}/child")]
async fn index(web::Path((comment_id)): web::Path<(u64)>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let comments = web::block(move || {
        let repository = CommentsRepositoryImpl::new(conn);
        let path = repository.get_path(comment_id);
        repository.select_comments(&path.unwrap().unwrap())
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let tree = Tree::new(&comments);
    Ok(HttpResponse::Ok().json(tree))
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

// fn main() {
    // let comments_repository = CommentsRepositoryImpl::new();
    // let path = String::from("1/");
    // let author = 5;

    // comments_repository.add_comments(1, &author, "hogehogehoge");
    // let path = comments_repository.get_path(1);
    // match path {
    //     Ok(n) => println!("path: {}", n.unwrap()),
    //     Err(e) => println!("e: {}", e)
    // }


    // let result = comments_repository.add_comments(1, &author, "hogehogehoge");
    // match result {
    //     _ => {}
    //     Err(e) => println!("error: {}", e),
    // }
    //
    // let result = comments_repository.select_comments(&path);
    // let tree = Tree::new(&result.unwrap());
    // let json = serde_json::to_string(&tree).unwrap();
    // println!("{}", json);

    // match result {
    //     Ok(n) => {
    //         let tree = Tree::new(&n);
    //         let json = serde_json::to_string(&tree).unwrap();
    //         println!("{}", json);
    //     }
    //     Err(e) => {
    //         println!("error: {}", e);
    //     }
    // }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // let connection = MysqlConnection::establish(&database_url)
    //     .expect(&format!("Error connecting to {}", database_url));

    // let repository = PostsRepositoryImpl::new();
    // let state = PostState::new(Box::new(PostsServiceImpl::new(&repository)));

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(index)
            // .service(get_posts)
            // .service(post_post)
            // .service(patch_post)
            // .service(delete_post)
    })
    .bind(&bind)?
    .run()
    .await
}

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
