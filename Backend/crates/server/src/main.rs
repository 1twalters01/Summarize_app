use actix_web::{App, HttpServer, web};
extern crate users;
pub mod accounts;
pub mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
          .service(web::resource("/").to(|| async {
                "Hello, world!"
            }))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
