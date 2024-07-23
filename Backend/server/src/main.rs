use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

pub mod middleware;
pub mod utils;
pub mod generated;

pub mod ping;
pub mod accounts;
// pub mod settings;
// pub mod subscriptions;

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
            .configure(ping::urls::config)
            .configure(accounts::urls::config)
            // .configure(settings::urls::config)
            // .configure(subscriptions::urls::config)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
