use crate::{middleware, ping::routes};
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ping")
        .route(
            "/post_book",
            post().to(views::books::example::post_book_id)
        ),
    );
}
