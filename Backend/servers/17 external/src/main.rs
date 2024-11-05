use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

pub mod generated;
pub mod routes;
pub mod queries;
pub mod views;
pub mod models;
pub mod datatypes;
pub mod middleware;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();

        App::new()
            .wrap(cors)
            .configure(routes::ping::config)
    })
    .bind("127.0.0.1:8017")?
    .run()
    .await
}

