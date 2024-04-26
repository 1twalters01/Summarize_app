use actix_web::{App, HttpServer};
use actix_cors::Cors;

pub mod accounts;
pub mod settings;
pub mod databases;
pub mod utils;
pub mod tokens;
pub mod ping;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();

        App::new()
            .wrap(cors)
            .configure(ping::urls::config)
            .configure(accounts::urls::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
