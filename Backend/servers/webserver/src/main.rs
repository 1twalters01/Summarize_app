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
        App::new()
            .route("/favicon.ico", get().to(utils::routes::favicon_ico))
            .route("/main.js", get().to(utils::routes::main_js))
            .route("/", get().to(utils::routes::main_html))
            .route("{param:.*[^.bundle.js]}", get().to(utils::routes::main_html))
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
