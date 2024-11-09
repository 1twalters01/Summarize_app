use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

pub mod datatypes;
pub mod generated;
pub mod middleware;
pub mod models;
pub mod queries;
pub mod routes;
pub mod services;
pub mod utils;
pub mod views;

pub mod accounts;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8001")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();

        App::new()
            .wrap(cors)
            .configure(routes::ping::config)
            .configure(routes::accounts::config)
            .configure(routes::settings::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
