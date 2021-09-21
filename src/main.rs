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

fn main() {
    // let connection = establish_connection();
    // let repository = repository::posts_repository_impl::new_posts_repository_impl(connection);

    let repository = PostsRepositoryImpl::new();
    let posts = repository.show_posts(true);

    for post in posts {
        println!("{}", post.title);
        println!("--------------\n");
        println!("{}", post.body);
    }
}
