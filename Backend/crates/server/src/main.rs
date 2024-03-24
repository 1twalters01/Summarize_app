use actix_web::{App, HttpServer};
use actix_cors::Cors;
extern crate users;
pub mod accounts;
pub mod settings;

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
            .service(accounts::routes::login)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
