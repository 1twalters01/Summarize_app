use crate::{middleware, views::settings};
use actix_web::web::{post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/settings/profile")
            .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
            .route(
                "/change-email",
                post().to(settings::profile::email::post_email),
            )
            .route(
                "/change-email/confirm",
                post().to(settings::profile::email::post_confirmation),
            )
            .route(
                "/change-username",
                post().to(settings::profile::username::post_username),
            )
            .route(
                "/change-username/confirm",
                post().to(settings::profile::username::post_confirmation),
            )
            .route(
                "/change-name",
                post().to(settings::profile::name::post_name),
            )
            .route(
                "/change-name/confirm",
                post().to(settings::profile::name::post_confirmation),
            )
            .route(
                "/change-password",
                post().to(settings::profile::password::post_password),
            )
            .route(
                "/change-password/confirm",
                post().to(settings::profile::password::post_confirmation),
            )
            .route(
                "/delete-account",
                post().to(settings::profile::delete_account::post_delete),
            ),
    );
}
