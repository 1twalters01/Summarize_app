use crate::{middleware, views::ping};
use actix_web::web::{post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ping")
        .route(
            "/post_book",
            post().to(ping::books::post_book_id)
        ),
    );
}
