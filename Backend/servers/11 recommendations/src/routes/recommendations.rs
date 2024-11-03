use crate::{views::recommendations, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/summary/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::summaries::update::update_cache))
        .route("/fetch", get().to(recommendations::summaries::fetch::recommendations))
        .route("/fetch/cached", get().to(recommendations::summaries::cached::get_cached))
        .route("/fetch/post", post().to(recommendations::summaries::filtered::get_filtered))
    )
    .service(
        scope("/book/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::books::update::update_cache))
        .route("/fetch", get().to(recommendations::books::fetch::recommendations))
        .route("/fetch/cached", get().to(recommendations::books::cached::get_cached))
        .route("/fetch/post", post().to(recommendations::books::filtered::get_filtered))
    )
    .service(
        scope("/author/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::authors::update::update_cache))
        .route("/fetch", get().to(recommendations::authors::fetch::recommendations))
        .route("/fetch/cached", get().to(recommendations::authors::cached::get_cached))
        .route("/fetch/post", post().to(recommendations::authors::filtered::get_filtered))
    )
    .service(
        scope("/summarizer/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::summarizers::update::update_cache))
        .route("/fetch", get().to(recommendations::summarizers::fetch::recommendations))
        .route("/fetch/cached", get().to(recommendations::summarizers::cached::get_cached))
        .route("/fetch/post", post().to(recommendations::summarizers::filtered::get_filtered))
    );
}