#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;

use std::env;

use actix_web::{App, get, HttpServer, web};
use actix_web::middleware::Logger;
use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use env_logger::Env;
use serde::{Deserialize, Serialize};

use crate::domain::model::tree::Tree;
use crate::domain::repository::comments_repository::CommentsRepository;
use crate::infrastructure::repository::comments_repository_impl::CommentsRepositoryImpl;

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

#[derive(Deserialize)]
struct TreeInfo {
    tree_id: u64,
}

#[get("/children/{tree_id}")]
async fn get_comments(server: web::Data<Server>, info: web::Path<TreeInfo>) -> String {
    let result = server.pool.get();
    match result {
        Ok(n) => {
            let repository = CommentsRepositoryImpl::new(n);
            let path_result = repository.get_path(info.tree_id);
            match path_result {
                Ok(n) => {
                    if let Some(p) = n {
                        let select_result = repository.select_comments(&p);
                        match select_result {
                            Ok(n) => {
                                let tree = Tree::new(&n);
                                serde_json::to_string(&tree).unwrap()
                            },
                            Err(e) => format!("Error Occurred. {}", e),
                        }
                    } else {
                        String::from("path parameter is null")
                    }
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

