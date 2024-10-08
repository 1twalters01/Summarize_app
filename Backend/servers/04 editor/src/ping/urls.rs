use crate::{middleware, ping::routes};
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/ping")
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/any_auth",
                web::get().to(routes::optional_authentication::ping_get_any_auth),
            )
            .route(
                "/any_auth",
                web::post().to(routes::optional_authentication::ping_post_any_auth),
            ),
    )
    .service(
        web::scope("/ping")
            .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/only_auth",
                web::get().to(routes::requires_authentication::ping_get_only_auth),
            )
            .route(
                "/only_auth",
                web::post().to(routes::requires_authentication::ping_post_only_auth),
            ),
    )
    .service(
        web::scope("/ping")
            .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
            .wrap(actix_web::middleware::Logger::default())
            .route(
                "/not_auth",
                web::get().to(routes::no_authentication::ping_get_not_auth),
            )
            .route(
                "/not_auth",
                web::post().to(routes::no_authentication::ping_post_not_auth),
            ),
    );
}
