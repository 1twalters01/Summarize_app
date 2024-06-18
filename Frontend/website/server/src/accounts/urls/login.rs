use actix_web::web::{get, post, scope, ServiceConfig};
use crate::accounts::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
        .service(routes::login::login)
        // .route(
        //     "/575.bundle.js",
        //     get().to(routes::login::login_totp)
        // )
        // .route(
        //     "/login/2fa",
        //     post().to(routes::login::login_totp_post)
        // ),
    );
}
