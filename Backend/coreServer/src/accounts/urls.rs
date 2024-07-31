use crate::{accounts::routes, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/register")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route("/email", post().to(routes::register::email::post_email))
            .route(
                "/verify",
                post().to(routes::register::verification::post_verify),
            )
            .route(
                "/verify/{register_email_token}/{verification_code}",
                post().to(routes::register::verification::link_verify),
            )
            .route(
                "/details",
                post().to(routes::register::details::post_details),
            ),
    )
    .service(
        scope("/login")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route("/email", post().to(routes::login::email::post_email))
            .route(
                "/password",
                post().to(routes::login::password::post_password),
            )
            .route("/totp", post().to(routes::login::totp::post_totp))
            .route("/refresh-token", post().to(routes::refresh::refresh_token)),
    )
    .service(
        scope("/password-reset")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route(
                "/email",
                post().to(routes::password_reset::email::post_email),
            )
            .route(
                "/verify",
                post().to(routes::password_reset::verification::post_verify),
            )
            .route(
                "/verify/{uidb64}/{verification_code}",
                post().to(routes::password_reset::verification::link_verify),
            )
            .route(
                "/password",
                post().to(routes::password_reset::password::post_password_reset),
            ),
    )
    .service(
        scope("/captcha")
            .route("/get", get().to(routes::captcha::get_captcha))
            .route("/verify", post().to(routes::captcha::verify_captcha)),
    )
    .service(
        scope("/oauth2")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            // make google oauth account be the same as logging in regularly if using a gmail
            .route("/authorise", post().to(routes::oauth2::authorise))
            .route("/callback", post().to(routes::oauth2::callback))
            .route("/refresh-token", post().to(routes::oauth2::refresh_token)),
    );
}
