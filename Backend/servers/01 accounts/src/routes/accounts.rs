use crate::{
    middleware::{
        authentication::{Authenticated, AuthenticationMiddlewareFactory, NotAuthenticated},
        verified_captcha::IsVerified as VerificationMiddleware,
    },
    views,
};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/register")
            .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
            .route(
                "/email",
                post().to(views::accounts::register::from_login::email::post_email),
            )
            .wrap(VerificationMiddleware)
            .route(
                "/verify",
                post().to(views::accounts::register::from_login::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(views::accounts::register::from_login::verification::link_verify),
            )
            .route(
                "/details",
                post().to(views::accounts::register::from_login::details::post_details),
            ),
    )
    .service(
        scope("/register/from-guest")
            .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
            .route(
                "/email",
                post().to(views::accounts::register::from_guest::email::post_email),
            )
            .wrap(VerificationMiddleware)
            .route(
                "/verify",
                post().to(views::accounts::register::from_guest::verification::post_verify),
            )
            .route(
                "/verify/{header_token}/{verification_code}",
                post().to(views::accounts::register::from_guest::verification::link_verify),
            )
            .route(
                "/details",
                post().to(views::accounts::register::from_guest::details::post_details),
            ),
    )
    .service(
        scope("/register/from-oauth")
            .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
            .route(
                "/confirmation",
                post().to(views::accounts::register::from_oauth::confirmation::post_confirmation)
            )
            .route(
                "/password",
                post().to(views::accounts::register::from_oauth::password::post_password)
            )
    )
    .service(
        scope("/login")
            .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
            .route("/guest", get().to(views::accounts::login::is_guest::guest::get_guest))
            .route(
                "/email",
                post().to(views::accounts::login::is_authenticated::email::post_email),
            )
            .wrap(VerificationMiddleware)
            .route(
                "/password",
                post().to(views::accounts::login::is_authenticated::password::post_password),
            )
            .route(
                "/totp",
                post().to(views::accounts::login::is_authenticated::totp::post_totp),
            ),
        // .route("/sms", post().to(views::accounts::login::is_authenticated::totp::post_sms))
        // .route("/biometrics", post().to(views::accounts::login::is_authenticated::totp::post_biometrics)),
    )
    .service(
        scope("/login")
            .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
            .wrap(VerificationMiddleware)
            .route(
                "/refresh-token",
                post().to(views::accounts::login::refresh::post_refresh_token),
            ),
    )
    .service(
        scope("/password-reset")
            .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
            .route(
                "/email",
                post().to(views::accounts::password_reset::email::post_email),
            )
            .wrap(VerificationMiddleware)
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
    )
    .service(
        scope("/captcha")
            .route("/get", get().to(views::accounts::captcha::get::get_captcha))
            .route(
                "/verify",
                post().to(views::accounts::captcha::verification::verify_captcha),
            ),
    );
    // .service(
    //     scope("/oauth2")
    //         .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
    //         // make google oauth account be the same as logging in regularly if using a gmail
    //         .route("/authorise", post().to(views::accounts::oauth2::authorise))
    //         .route("/callback", post().to(views::accounts::oauth2::callback))
    //         .route("/refresh-token", post().to(views::accounts::oauth2::refresh_token)),
    // );
}
