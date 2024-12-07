use crate::{views::recommendations, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ruler")
        .wrap(middleware::authentication)
        .route(
            "/toggle-account",
            post().to(views::reader::place::ruler)
        )
        .route(
            "/toggle-device-type",
            post().to(views::reader::place::ruler)
        )
        .route(
            "/toggle-device",
            post().to(views::reader::place::ruler)
        )
    )
}