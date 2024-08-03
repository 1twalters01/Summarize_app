use crate::user_data::routes;
use actix_web::web::{post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/data")
        .route("/genres/example", post().to(routes::genres::example::get_genres))
        .route("/libraries/example", post().to(routes::libraries::example::get_libraries))
    );
}
