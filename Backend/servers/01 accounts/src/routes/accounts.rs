use crate::{views::accounts, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/register")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route("/email", post().to(accounts::register::email::post_email))
            .route(
                "/verify",
                post().to(accounts::register::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(accounts::register::verification::link_verify),
            )
            .route(
                "/details",
                post().to(accounts::register::details::post_details),
            ),
    )
    .service(
        scope("/login")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route("/email", post().to(accounts::login::email::post_email))
            .route(
                "/password",
                post().to(accounts::login::password::post_password),
            )
            .route("/totp", post().to(accounts::login::totp::post_totp))
            // .route("/refresh-token", post().to(accounts::refresh::refresh_token)),
    )
    .service(
        scope("/password-reset")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route(
                "/email",
                post().to(accounts::password_reset::email::post_email),
            )
            .route(
                "/verify",
                post().to(accounts::password_reset::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(accounts::password_reset::verification::link_verify),
            )
            .route(
                "/password",
                post().to(accounts::password_reset::password::post_password_reset),
            ),
    );
    // .service(
    //     scope("/captcha")
    //         .route("/get", get().to(accounts::captcha::get_captcha))
    //         .route("/verify", post().to(accounts::captcha::verify_captcha)),
    // )
    // .service(
    //     scope("/oauth2")
    //         .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
    //         // make google oauth account be the same as logging in regularly if using a gmail
    //         .route("/authorise", post().to(accounts::oauth2::authorise))
    //         .route("/callback", post().to(accounts::oauth2::callback))
    //         .route("/refresh-token", post().to(accounts::oauth2::refresh_token)),
    // );
}
