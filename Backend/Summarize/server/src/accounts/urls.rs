use actix_web::web::ServiceConfig;
use crate::accounts::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(routes::register_email)
        .service(routes::register_verify)
        .service(routes::register_verify_link)
        .service(routes::register_details)
        .service(routes::login_email)
        .service(routes::login_password)
        .service(routes::login_totp)
        .service(routes::password_reset)
        .service(routes::password_reset_confirm);
}
