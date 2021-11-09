#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;

use std::borrow::Borrow;
use std::env;
use std::error::Error;
use std::ops::Deref;

use actix_web::{App, delete, get, HttpResponse, HttpServer, patch, post, Responder, web};
use actix_web::error::ParseError::Method;
use actix_web::middleware::Logger;
use actix_web::web::Query;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use env_logger::Env;
use serde::{Deserialize, Serialize};
use serde_json::ser::State;

use infrastructure::repository::posts_repository_impl::PostsRepositoryImpl;
use rust_web_app::schema::CommentsPE::dsl::CommentsPE;

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

#[derive(Clone)]
struct Server {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Server { pool }
    }
}

#[get("/comments")]
async fn get_comments(server: web::Data<Server>) -> String {
    let result = server.pool.get();
    match result {
        Ok(n) => {
            let repository = CommentsRepositoryImpl::new(n);
            let path = String::from("1/");
            let select_result = repository.select_comments(&path);
            match select_result {
                Ok(n) => {
                    let tree = Tree::new(&n);
                    serde_json::to_string(&tree).unwrap()
                },
                Err(e) => format!("Error Occurred. {}", e),
            }
        },
        Err(e) => format!("Error Occurred. {}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = "127.0.0.1:8080";
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("date=%t\tip=%a\tstatus_code=%s\tduration=%D"))
            .data(Server::new())
            .service(get_comments)
    })
    .bind(&bind)?
    .run()
    .await
}

