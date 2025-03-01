use crate::{
    middleware::{
        authentication::{Authenticated, AuthenticationMiddlewareFactory, NotAuthenticated},
        verified_captcha::VerificationMiddleware,
        logger::LoggerMiddleware,
    },
    views,
};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    let logger_enabled = true;

    cfg.service(
        scope("/register")
            .wrap(LoggerMiddleware { logger_enabled })
            .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
            // .wrap(RateLimiter { limit: 3, expiry_in_seconds: Some(5) })
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
            .wrap(LoggerMiddleware { logger_enabled })
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
            .wrap(LoggerMiddleware { logger_enabled })
            .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
            .route(
                "email",
                post().to(views::accounts::register::from_oauth::initiate::post_email)
            )
            .wrap(VerificationMiddleware)
            .route(
                "/verification",
                post().to(views::accounts::register::from_oauth::verification::post_verification)
            )
            .route(
                "/password",
                post().to(views::accounts::register::from_oauth::password::post_password)
            )
    )
    .service(
        scope("/login")
            .wrap(LoggerMiddleware { logger_enabled })
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
            // .route(
            //     "/switch-authentication-method",
            //     get().to(views::accounts::login::is_authenticated::authentication_options::get_authentication_options)
            // )
            // .route(
            //     "/switch-authentication-method",
            //     post().to(views::accounts::login::is_authenticated::switch_authentication::post_authentication_method)
            // )
            .route(
                "/totp",
                post().to(views::accounts::login::is_authenticated::totp::post_totp),
            ),
            .route(
                "/sms",
                post().to(views::accounts::login::is_authenticated::totp::post_sms)
            )
            .route(
                "/biometrics",
                post().to(views::accounts::login::is_authenticated::totp::post_biometrics)
            ),
    )
    .service(
        scope("/login/oauth2/google")
            .wrap(LoggerMiddleware { logger_enabled })
            .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
            .route(
                "/google/authorise",
                post().to(views::accounts::login::oauth2::google::authorise)
            )
            .route(
                "/google/callback",
                post().to(views::accounts::login::oauth2::google::callback)
            )
            .route(
                "/google/authorise",
                post().to(views::accounts::login::oauth2::apple::authorise)
            )
            .route(
                "/google/callback",
                post().to(views::accounts::login::oauth2::apple::callback)
            )
    );
    .service(
        scope("/login")
            .wrap(LoggerMiddleware { logger_enabled })
            .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
            .wrap(VerificationMiddleware)
            .route(
                "/refresh-token",
                post().to(views::accounts::login::refresh_token::post_refresh_token),
            ),
    )
    .service(
        scope("/password-reset")
            .wrap(LoggerMiddleware { logger_enabled })
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
            .wrap(LoggerMiddleware { logger_enabled })
            .route(
                "/get",
                get().to(views::accounts::captcha::get::get_captcha)
            )
            .route(
                "/verify",
                post().to(views::accounts::captcha::verification::verify_captcha),
            ),
    );
}
