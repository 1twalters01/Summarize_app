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
        let linux_dir: String = env::var("LINUX_DIR").unwrap();
        App::new()
            .route("/update/check", post().to(updates::validations::views::check))
            .route("/update/lua", post().to(updates::lua::views::post_specific))
            .route("/update/lua/all", get().to(updates::lua::views::get_all))
            .route("/update/c", post().to(updates::c::views::post_specific))
            .route("/update/c/all", get().to(updates::c::views::get_all))
            .route("/update/validate", get().to(updates::validations::views::validate))

            // Require admin access
            .route("update/lua/push", post().to(updates::lua::views::post_push))
            .route("update/c/push", post().to(updates::c::views::post_push))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
