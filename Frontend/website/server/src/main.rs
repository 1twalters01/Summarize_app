mod index;
mod accounts;
mod app;
mod settings;
mod datatypes;
mod utils;
// extern crate users;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(utils::routes::cookies)
            .service(index::routes::get_routes)

            .service(index::routes::favicon)
            .service(index::routes::index_html)
            .service(index::routes::index_css)
            .service(index::routes::index_js)

            .service(index::routes::pricing_html)
            .service(index::routes::pricing_js)

            .service(index::routes::download_html)
            .service(index::routes::download_js)
            .service(index::routes::download_web_clipper_html)
            .service(index::routes::download_web_clipper_js)
            .service(index::routes::download_mobile_html)
            .service(index::routes::download_mobile_js)
            .service(index::routes::download_desktop_html)
            .service(index::routes::download_desktop_js)

            .service(index::routes::library_html)
            .service(index::routes::library_js)
            .service(index::routes::community_html)
            .service(index::routes::community_js)
            .service(index::routes::sync_html)
            .service(index::routes::sync_js)

            .service(index::routes::terms_html)
            .service(index::routes::terms_js)
            .service(index::routes::terms_pdf)
            .service(index::routes::privacy_html)
            .service(index::routes::privacy_js)
            .service(index::routes::privacy_pdf)
            
            .service(index::routes::blog_html)
            .service(index::routes::blog_js)
            .service(index::routes::ai_html)
            .service(index::routes::ai_js)

            .service(index::routes::releases_html)
            .service(index::routes::releases_js)
            .service(index::routes::email_us_html)
            .service(index::routes::email_us_js)

            .service(index::routes::main_js)

            .service(accounts::routes::login)
            .service(accounts::routes::login_totp)
            .service(accounts::routes::logout)
            .service(accounts::routes::register)
            .service(accounts::routes::register_verify)
            .service(accounts::routes::activate)
            .service(accounts::routes::username_reset)
            .service(accounts::routes::username_reset_token)
            .service(accounts::routes::password_reset)
            .service(accounts::routes::password_reset_token)

            .service(settings::routes::change_username)
            .service(settings::routes::change_password)
            .service(settings::routes::change_email)
            .service(settings::routes::change_theme)
            .service(settings::routes::close_account)
            .service(settings::routes::two_factor_auth)

            .service(
                web::scope("/accounts")
                .service(accounts::routes::get_routes)
                .service(accounts::routes::main_html)
            )
            .service(
                web::scope("/settings")
                .service(settings::routes::get_routes)
                // .service(settings::routes::main_html)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
