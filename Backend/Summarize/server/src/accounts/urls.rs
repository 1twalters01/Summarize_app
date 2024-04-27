use actix_web::web::ServiceConfig;
use crate::accounts::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(routes::register_email)
        .service(routes::login_email)
        .service(routes::login_password)
        .service(routes::login_totp)
        .service(routes::registerVerify)
        .service(routes::registerDetails)
        .service(routes::password_reset)
        .service(routes::password_reset_confirm);
}
