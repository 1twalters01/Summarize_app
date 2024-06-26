mod index;
mod accounts;
mod app;
mod settings;
mod datatypes;
mod utils;
use std::{fs, path::PathBuf};
use actix_files as files;

use actix_web::{web::{get, route, Bytes}, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index::routes::main_js)
            .route(
                "/",
                get().to(index::routes::main_html)
            )
            .route(
                "{param:.*[^.bundle.js]}",
                get().to(index::routes::main_html)
            )
            .service(files::Files::new("", "../content/dist/main/javascript"))
            // .configure(index::urls::config)
            // .configure(accounts::urls::config)
            .default_service(
                route().to(|| async {
                    let path: PathBuf = "../content/dist/main/index.html".into();
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
