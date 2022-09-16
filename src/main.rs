mod controllers;
mod database;
mod middlewares;
mod model;
mod utils;

extern crate dotenv;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use controllers::{
    blog::{create, get_blog, get_blogs, upload, update},
    user::{hello, login, register},
};
use database::{create_blog_index, create_init_user};
use dotenv::dotenv;
use env_logger::Env;
use mongodb::Client;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_url = env::var("MONGO_URL").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(mongo_url)
        .await
        .expect("failed to connect");
    
    create_blog_index(&client).await;
    create_init_user(&client).await;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./static"))
            .app_data(web::Data::new(client.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .route("/", web::get().to(hello))
            // .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/blog", web::post().to(create))
            .route("/blog", web::patch().to(update))
            .route("/blog", web::get().to(get_blog))
            .route("/blogs", web::get().to(get_blogs))
            .route("/upload", web::post().to(upload))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
