use crate::{accounts::views, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/register")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route("/email", post().to(views::register::email::post_email))
            .route(
                "/verify",
                post().to(views::register::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(views::register::verification::link_verify),
            )
            .route(
                "/details",
                post().to(views::register::details::post_details),
            ),
    )
    .service(
        scope("/login")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route("/email", post().to(views::login::email::post_email))
            .route(
                "/password",
                post().to(views::login::password::post_password),
            )
            .route("/totp", post().to(views::login::totp::post_totp))
            .route("/refresh-token", post().to(views::refresh::refresh_token)),
    )
    .service(
        scope("/password-reset")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route(
                "/email",
                post().to(views::password_reset::email::post_email),
            )
            .route(
                "/verify",
                post().to(views::password_reset::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(views::password_reset::verification::link_verify),
            )
            .route(
                "/password",
                post().to(views::password_reset::password::post_password_reset),
            ),
    )
    .service(
        scope("/captcha")
            .route("/get", get().to(views::captcha::get_captcha))
            .route("/verify", post().to(views::captcha::verify_captcha)),
    )
    .service(
        scope("/oauth2")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            // make google oauth account be the same as logging in regularly if using a gmail
            .route("/authorise", post().to(views::oauth2::authorise))
            .route("/callback", post().to(views::oauth2::callback))
            .route("/refresh-token", post().to(views::oauth2::refresh_token)),
    );
}
