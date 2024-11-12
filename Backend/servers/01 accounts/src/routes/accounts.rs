use crate::{middleware, views};
use actix_web::web::{post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/register")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route(
                "/email",
                post().to(views::accounts::register::email::post_email),
            )
            .route(
                "/verify",
                post().to(views::accounts::register::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(views::accounts::register::verification::link_verify),
            )
            .route(
                "/details",
                post().to(views::accounts::register::details::post_details),
            ),
    )
    .service(
        scope("/login")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route(
                "/email",
                post().to(views::accounts::login::email::post_email),
            )
            .route(
                "/password",
                post().to(views::accounts::login::password::post_password),
            )
            .route("/totp", post().to(views::accounts::login::totp::post_totp))
            .route(
                "/refresh-token",
                post().to(views::accounts::login::refresh::post_refresh_token),
            ),
    )
    .service(
        scope("/password-reset")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .route(
                "/email",
                post().to(views::accounts::password_reset::email::post_email),
            )
            .route(
                "/verify",
                post().to(views::accounts::password_reset::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(views::accounts::password_reset::verification::link_verify),
            )
            .route(
                "/password",
                post().to(views::accounts::password_reset::password::post_password_reset),
            ),
    );
    // .service(
    //     scope("/captcha")
    //         .route("/get", get().to(views::accounts::captcha::get_captcha))
    //         .route("/verify", post().to(views::accounts::captcha::verification_captcha)),
    // )
    // .service(
    //     scope("/oauth2")
    //         .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
    //         // make google oauth account be the same as logging in regularly if using a gmail
    //         .route("/authorise", post().to(views::accounts::oauth2::authorise))
    //         .route("/callback", post().to(views::accounts::oauth2::callback))
    //         .route("/refresh-token", post().to(views::accounts::oauth2::refresh_token)),
    // );
}
