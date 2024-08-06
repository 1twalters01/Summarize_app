use crate::{middleware, settings::routes};
use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/profile")
            .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
            .service(routes::profile::email::change_email)
            .service(routes::profile::change_name)
            .service(routes::profile::change_username)
            .service(routes::profile::change_password),
    );
}
