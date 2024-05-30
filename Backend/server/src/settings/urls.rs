use crate::settings::routes;
use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/profile")
            .service(routes::profile::change_name)
            .service(routes::profile::change_email)
            .service(routes::profile::change_username)
            .service(routes::profile::change_password),
    );
}
