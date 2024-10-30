use crate::{middleware, views::ping};
use actix_web::web::{self, ServiceConfig};

/// A set of ping that serve as ping to hit when checking the health of the service.
///
/// Contains get request route functions for the following:
///     Requires authentication,
///     Requires no authentication,
///     Allows any authentication state,
///
/// Plans are in place to allow for:
///     Requires admin priviliges
///     Requires admin or being in a certain group
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/ping")
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/any_auth",
                web::get().to(ping::optional_authentication::ping_get_any_auth),
            )
            .route(
                "/any_auth",
                web::post().to(ping::optional_authentication::ping_post_any_auth),
            ),
    )
    .service(
        web::scope("/ping")
            .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/only_auth",
                web::get().to(ping::requires_authentication::ping_get_only_auth),
            )
            .route(
                "/only_auth",
                web::post().to(ping::requires_authentication::ping_post_only_auth),
            ),
    )
    .service(
        web::scope("/ping")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/not_auth",
                web::get().to(ping::no_authentication::ping_get_not_auth),
            )
            .route(
                "/not_auth",
                web::post().to(ping::no_authentication::ping_post_not_auth),
            ),
    );
}
