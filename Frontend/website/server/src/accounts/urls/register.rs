use actix_web::web::{scope, ServiceConfig};
use crate::accounts::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
            .service(routes::register::register)
            .service(routes::register::register_email)
            .service(routes::register::register_verification)
            .service(routes::register::register_verification_link)
            .service(routes::register::register_details)
    );
}

