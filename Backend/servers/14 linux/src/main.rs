use actix_web::{web::{get, route, Bytes}, App, HttpResponse, HttpServer};
use actix_files as files;
use std::{env, fs, path::PathBuf};
use dotenv::dotenv;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let website_dir: String = env::var("WEBSITE_DIR").unwrap();

    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
