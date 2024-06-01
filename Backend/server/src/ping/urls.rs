use crate::ping::routes;
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/ping")
        .route("/any_auth", web::get().to(routes::ping_get_any_auth))
        .route("/any_auth", web::post().to(routes::ping_post_any_auth))
        .route("/only_auth", web::get().to(routes::ping_get_only_auth))
        .route("/only_auth", web::post().to(routes::ping_post_only_auth))
        .route("/not_auth", web::get().to(routes::ping_get_not_auth))
        .route("/not_auth", web::post().to(routes::ping_post_not_auth))
    );
}
