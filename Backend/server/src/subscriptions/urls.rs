use crate::{middleware, subscriptions::routes};
use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/profile")
            .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
            .service(routes::profile::change_password),
    );
}

