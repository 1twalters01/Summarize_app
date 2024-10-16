use actix_web::{web::{get, post, route, Bytes}, App, HttpResponse, HttpServer};
use actix_files as files;
use std::{env, fs, path::PathBuf};
use dotenv::dotenv;
mod utils;
mod updates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let ios_dir: String = env::var("IOS_DIR").unwrap();
        App::new()
            .route("/update/check", post().to(updates::validations::views::check))
            .route("/update/validate", get().to(updates::validations::views::validate))
    })
    .bind("127.0.0.1:8016")?
    .run()
    .await
}
