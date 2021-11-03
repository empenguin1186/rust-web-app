#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;

use std::borrow::Borrow;
use std::env;
use std::error::Error;
use std::ops::Deref;

use actix_web::{App, delete, get, HttpResponse, HttpServer, patch, post, Responder, web};
use actix_web::web::Query;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};
use serde_json::ser::State;

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

#[derive(Serialize, Deserialize)]
struct CommentsResponse {
    results: Option<Tree>,
    error: Option<String>,
}

struct MyApp {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

#[get("/comments")]
async fn get_comments(app: web::Data<MyApp>) -> impl Responder {
    let connection = app.pool.get()?;
    // TODO まだ動かない
    let repository = CommentsRepositoryImpl::new(connection.deref().to_owned());
    let path = "1/".to_string();
    let result = app.repository.select_comments(&path);
    match result {
        Ok(n) => HttpResponse::Ok().json(CommentsResponse {
            results: Some(Tree::new(&n)),
            error: None,
        }),
        Err(e) => HttpResponse::NotFound().json(CommentsResponse {
            results: None,
            error: Some(format!("{:?}", e)),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let bind = "127.0.0.1:8080";

    HttpServer::new(|| {
        App::new()
            .data(MyApp{ pool })
            .service(get_comments)
    })
    .bind(&bind)?
    .run()
    .await
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
