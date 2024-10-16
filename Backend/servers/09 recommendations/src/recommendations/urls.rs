use crate::recommendations::views;
use actix_web::web::{post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("recommendations").route("/example", post().to(views::books::example::post_book_id)),
    );
}
