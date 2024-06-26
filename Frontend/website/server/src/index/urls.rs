use actix_web::web::{get, scope, ServiceConfig};
use crate::index::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg
        .service(routes::index)
        .service(routes::pricing)
        .route("/get-routes", get().to(routes::get_routes));
}


// pub fn config(cfg: &mut ServiceConfig) {
//     cfg.service(
//         scope("")
//         .service(routes::index)
//         .service(routes::pricing)
//         .route("/get-routes", get().to(routes::get_routes))
//     );
//
//
//         // .service(utils::routes::cookies)
//         //
//         // .service(index::routes::favicon)
//         // .service(index::routes::index_css)
//         //
//         // .service(index::routes::pricing_html)
//         //
//         // .service(index::routes::downloads::download_html)
//         // .service(index::routes::downloads::download_js)
//         // .service(index::routes::downloads::download_web_clipper_html)
//         // .service(index::routes::downloads::download_web_clipper_js)
//         // .service(index::routes::downloads::download_mobile_html)
//         // .service(index::routes::downloads::download_mobile_js)
//         // .service(index::routes::downloads::download_desktop_html)
//         // .service(index::routes::downloads::download_desktop_js)
//         //
//         // .service(index::routes::library_html)
//         // .service(index::routes::community_html)
//         // .service(index::routes::sync_html)
//         //
//         // .service(index::routes::terms_html)
//         // .service(index::routes::terms_pdf)
//         // .service(index::routes::privacy_html)
//         // .service(index::routes::privacy_pdf)
//         //
//         // .service(index::routes::blog_html)
//         // .service(index::routes::ai_html)
//         //
//         // .service(index::routes::releases_html)
//         // .service(index::routes::email_us_html)
// }
//
//
