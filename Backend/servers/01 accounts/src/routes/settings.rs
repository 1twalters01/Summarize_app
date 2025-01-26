use crate::{
    middleware::{
        authentication::{Authenticated, AuthenticationMiddlewareFactory},
        verified_captcha::IsVerified as VerificationMiddleware,
    }
    views::settings,
};
use actix_web::web::{post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/settings/profile")
            .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
            .wrap(VerificationMiddleware)
            .route(
                "/change-email",
                post().to(settings::profile::change_email::email::post_email),
            )
            .route(
                "/change-email/confirm",
                post().to(settings::profile::change_email::confirmation::post_confirmation),
            )
            // .route(
            //     "change_language",
            //     post().to(settings::profile::change_language::language::post_language)
            // )
            // .route(
            //     "change_language/confirmation",
            //     post().to(settings::profile::change_language::confirmation::post_confirmation)
            // )
            .route(
                "/change-name",
                post().to(settings::profile::change_name::name::post_name),
            )
            .route(
                "/change-name/confirm",
                post().to(settings::profile::change_name::confirmation::post_confirmation),
            )
            .route(
                "/change-password",
                post().to(settings::profile::change_password::password::post_password),
            )
            .route(
                "/change-password/confirm",
                post().to(settings::profile::change_password::confirmation::post_confirmation),
            )
            // .route(
            //     "/change_theme",
            //     post().to(settings::profile::change_theme::theme::post_theme),
            // )
            .route(
                "/change-username",
                post().to(settings::profile::change_username::username::post_username),
            )
            .route(
                "/change-username/confirm",
                post().to(settings::profile::change_username::confirmation::post_confirmation),
            )
            .route(
                "/delete-account",
                post().to(settings::profile::delete_account::delete_account::post_delete),
            ),
            // .route(
            //     "/oauth2/authorise",
            //     post().to(settings::profile::oauth::authorise::authorise),
            // )
            // .route(
            //     "/oauth2/callback",
            //     post().to(settings::profile::oauth::authorise::callback),
            // )
            // .route(
            //     "/oauth2/revoke",
            //     post().to(settings::profile::oauth::revoke::revoke),
            // )
            // .route(
            //     "/select-default-mfa",
            //     post().to(settings::profile::select_default_mfa::default::post_default)
            // )
            // .route(
            //     "toggle-biometrics",
            //     post().to(settings::profile::toggle_biometrics::biometrics)
            // )
            // .route(
            //     "toggle-biometrics/confirm",
            //     post().to(settings::profile::toggle_biometrics::confirmation::post_confirmation),
            // )
            // .route(
            //     "toggle-sms",
            //     post().to(settings::profile::toggle_sms::sms)
            // )
            // .route(
            //     "toggle-sms/confirm",
            //     post().to(settings::profile::toggle_sms::confirmation::post_confirmation),
            // )
            // .route(
            //     "toggle-totp",
            //     post().to(settings::profile::toggle_totp::totp)
            // )
            // .route(
            //     "toggle-totp/confirm",
            //     post().to(settings::profile::toggle_totp::confirmation::post_confirmation),
            // )
    );
}
