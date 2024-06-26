use actix_web::web::{get, scope, ServiceConfig};
use crate::accounts::routes;

pub fn config(cfg: &mut ServiceConfig) {
    cfg
        .service(routes::main::navbar)
        .service(routes::register::register) 
        .service(routes::register::register_email)
        .service(routes::register::register_verification)
        .service(routes::register::register_verification_link)
        .service(routes::register::register_details)
        .service(routes::login::login)
        .service(routes::login::login_email)
        .service(routes::login::login_password)
        .service(routes::login::login_totp)
        .route("/accounts/get-routes", get().to(routes::main::get_routes));
}

// pub fn config(cfg: &mut ServiceConfig) {
//     cfg
//         .service(
//             scope("")
//         .service(routes::main::navbar)
//         .service(routes::register::register)
//         .service(routes::register::register_email)
//         .service(routes::register::register_verification)
//         .service(routes::register::register_verification_link)
//         .service(routes::register::register_details)
//         .service(routes::login::login)
//         .service(routes::login::login_email)
//         .service(routes::login::login_password)
//         .service(routes::login::login_totp)
//         .route("/accounts/get-routes", get().to(routes::main::get_routes))
//         );
// }

