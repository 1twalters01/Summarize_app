use actix_web::web::{get, scope, ServiceConfig};
use crate::accounts::{urls, routes};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("")
        .service(routes::main::navbar)
        .service(
            scope("/accounts")
            .route(
                "/get-routes",
                get().to(routes::main::get_routes)
            )
            .route(
                "/{param:.*?}",
                get().to(routes::main::main_html)
            )
        )
        .configure(urls::register::config)
        .configure(urls::login::config)
    );
}

