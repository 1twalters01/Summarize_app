use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

pub mod generated;
pub mod middleware;
pub mod ping;
pub mod recommendations;
// pub mod settings;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8008")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();

        App::new()
            .wrap(cors)
            .configure(ping::urls::config)
            // .configure(settings::urls::config)
            .configure(recommendations::urls::config)
    })
    .bind("127.0.0.1:8006")?
    .run()
    .await
}
