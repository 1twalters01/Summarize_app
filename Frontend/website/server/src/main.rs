mod home;
mod accounts;
mod app;
mod settings;
mod datatypes;
// extern crate users;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home::routes::get_routes)
            .service(home::routes::favicon)
            .service(home::routes::index_html)
            .service(home::routes::index_css)
            .service(home::routes::index_js)

            .service(home::routes::pricing_html)
            .service(home::routes::pricing_js)

            .service(home::routes::download_html)
            .service(home::routes::download_js)
            .service(home::routes::download_web_clipper_html)
            .service(home::routes::download_web_clipper_js)
            .service(home::routes::download_mobile_html)
            .service(home::routes::download_mobile_js)
            .service(home::routes::download_desktop_html)
            .service(home::routes::download_desktop_js)

            .service(home::routes::library_html)
            .service(home::routes::library_js)
            .service(home::routes::community_html)
            .service(home::routes::community_js)
            .service(home::routes::sync_html)
            .service(home::routes::sync_js)

            .service(home::routes::terms_html)
            .service(home::routes::terms_js)
            .service(home::routes::terms_pdf)
            .service(home::routes::privacy_html)
            .service(home::routes::privacy_js)
            .service(home::routes::privacy_pdf)
            

            .service(home::routes::blog_html)
            .service(home::routes::blog_js)
            .service(home::routes::ai_html)
            .service(home::routes::ai_js)

            .service(home::routes::releases_html)
            .service(home::routes::releases_js)
            .service(home::routes::email_us_html)
            .service(home::routes::email_us_js)

            .service(home::routes::main_js)
            // .service(home::routes::main_html)

            .service(accounts::routes::login)
            .service(accounts::routes::login_totp)
            .service(accounts::routes::logout)
            .service(accounts::routes::register)
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
                .service(settings::routes::main_html)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
