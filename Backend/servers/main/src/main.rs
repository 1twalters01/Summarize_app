use actix_web::{App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();

        App::new()
            .wrap(cors)
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
