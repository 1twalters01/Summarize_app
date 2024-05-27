use actix_web::web::{ServiceConfig, scope};
use crate::accounts::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/register")
        .service(routes::register::post_email)
        .service(routes::register::post_verify)
        .service(routes::register::link_verify)
        .service(routes::register::post_details)
    )
    .service(scope("/login")
        .service(routes::login::post_email)
        .service(routes::login::post_password)
        .service(routes::login::post_totp)
        .service(routes::login::refresh_token)
    )
    .service(scope("/password-reset")
        .service(routes::password_reset::post_email)
        .service(routes::password_reset::post_verify)
        .service(routes::password_reset::link_verify)
        .service(routes::password_reset::post_password_reset)
    )
    .service(scope("/oauth2")
        // make google oauth account be the same as logging in regularly if using a gmail
        .service(routes::oauth2::authorise)
        .service(routes::oauth2::callback)
        .service(routes::oauth2::refresh)
    );
}

