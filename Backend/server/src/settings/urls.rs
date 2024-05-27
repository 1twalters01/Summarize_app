use actix_web::web::{ServiceConfig, scope};
use crate::settings::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/profile")
        .service(routes::profile::)
    );
}
