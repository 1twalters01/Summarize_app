use crate::{middleware, ping::routes};
use actix_web::web::{self, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/settings/recommendations")
        .wrap(middleware::authentication::requires_authenticated::Authenticated)
    );
}