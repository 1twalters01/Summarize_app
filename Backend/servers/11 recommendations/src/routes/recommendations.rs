use crate::{views::recommendations, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/summary/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::summaries::update::update_cached_summaries))
        .route("/fetch/cached", get().to(recommendations::summaries::cached::get_cached_summaries))
        .route("/fetch", get().to(recommendations::summaries::fetch::get_recommended_summaries))
        .route("/fetch/filtered", post().to(recommendations::summaries::filtered::get_filtered_summaries))
    )
    .service(
        scope("/book/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::books::update::update_cached_books))
        .route("/fetch/cached", get().to(recommendations::books::cached::get_cached_books))
        .route("/fetch", get().to(recommendations::books::fetch::get_recommended_books))
        .route("/fetch/filtered", post().to(recommendations::books::filtered::get_filtered_books))
    )
    .service(
        scope("/author/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::authors::update::update_cached_authors))
        .route("/fetch/cached", get().to(recommendations::authors::cached::get_cached_authors))
        .route("/fetch", get().to(recommendations::authors::fetch::get_recommended_authors))
        .route("/fetch/filtered", post().to(recommendations::authors::filtered::get_filtered_authors))
    )
    .service(
        scope("/summarizer/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
        .route("/update", get().to(recommendations::summarizers::update::update_cached_summarizers))
        .route("/fetch/cached", get().to(recommendations::summarizers::cached::get_cached_summarizers))
        .route("/fetch", get().to(recommendations::summarizers::fetch::get_recommended_summarizers))
        .route("/fetch/filtered", post().to(recommendations::summarizers::filtered::get_filtered_summarizers))
    );
}