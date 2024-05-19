use crate::ping::routes;
use actix_web::web::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(routes::ping_get_any_auth)
        .service(routes::ping_get_only_auth)
        .service(routes::ping_get_not_auth)
        .service(routes::ping_post_any_auth)
        .service(routes::ping_post_only_auth)
        .service(routes::ping_post_not_auth);
}
