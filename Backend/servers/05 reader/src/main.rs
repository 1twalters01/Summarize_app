use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

pub mod reader;
pub mod generated;
pub mod middleware;
pub mod ping;
pub mod settings;
pub mod utils;

async fn main() {
    dotenv().ok();
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8006")
            .allow_any_header()
            .allow_any_method()
            .expose_any_header();

        App::new()
            .wrap(cors)
            .configure(ping::urls::config)
            .configure(reader::urls::config)
            .configure(settings::urls::config)
    })
    .bind("127.0.0.1:8005")?
    .run()
    .await
}
