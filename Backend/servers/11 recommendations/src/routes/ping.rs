use crate::{middleware, views::ping};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ping")
        .route(
            "/get_book",
            get().to(ping::get::books::get_books)
        )
        .route(
            "/post_book",
            post().to(ping::post::books::post_books)
        ),
    );
}
