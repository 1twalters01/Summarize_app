use actix_web::{App,  HttpServer};
use dotenv::dotenv;

mod views;
mod datatypes;
mod routes;
mod utils;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();
//
//     HttpServer::new(|| {
//         App::new()
//             .configure(routes::ping::config)
//             .configure(routes::website::config)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

use actix_web::{web::{get, route, Bytes}, HttpResponse};
use actix_files as files;
use std::{env, fs, path::PathBuf};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let website_dir: String = env::var("WEBSITE_DIR").unwrap();
        App::new()
            .route("/favicon.ico", get().to(views::website::favicon::favicon_ico))
            .route("/main.js", get().to(views::website::main_js::main_js))
            .route("/", get().to(views::website::main_html::main_html))
            .route("{param:.*[^.bundle.js]}", get().to(views::website::main_html::main_html))
            .service(files::Files::new("", format!("{}/dist/main/javascript", website_dir)))
            .default_service(
                route().to(|| async {
                    let website_dir: String = env::var("WEBSITE_DIR").unwrap();
                    let path: PathBuf = format!("{}/dist/main/index.html", website_dir).into();
                    let data = Bytes::from(fs::read(&path).unwrap());

                    HttpResponse::Ok()
                        .content_type("text/html; charset=UTF-8")
                        .body(data)
                })
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
