// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello, World!")
// }
//
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey, there!")
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

#[macro_use]
extern crate diesel;
extern crate dotenv;
mod domain;
pub mod models;
mod repository;
pub mod schema;
use domain::posts_repository::PostsRepository;
use repository::posts_repository_impl::PostsRepositoryImpl;
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    let repository = PostsRepositoryImpl::new();
    delete_post(repository)
}

fn show_posts(repository: PostsRepositoryImpl, is_published: bool) {
    let posts = repository.show_posts(is_published);

    for post in posts {
        println!("{}", post.title);
        println!("--------------\n");
        println!("{}", post.body);
    }
}

fn write_post(repository: PostsRepositoryImpl) {
    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("\nOk! Let's write {} (Press {} when finished\n", title, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    repository.write_post(title, &body);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

fn publish_post(repository: PostsRepositoryImpl) {
    let update_id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("invalid id");

    repository.publish_post(update_id);
}

fn delete_post(repository: PostsRepositoryImpl) {
    let target = args().nth(1).expect("Expected a target to match against");

    repository.delete_post(&target);
}
