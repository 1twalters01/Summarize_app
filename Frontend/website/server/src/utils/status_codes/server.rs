use actix_web::{get, Responder, Result, web::Json};
use crate::utils::status_codes::datatypes::{Route, Method};

#[get("/get-routes")]
async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 6] = [
        Route::from(
            String::from("/error-500/"),
            Vec::from([Method::Get]),
            String::from("Error 500 - Internal Server Error")),
        Route::from(
            String::from("/error-501/"),
            Vec::from([Method::Get]),
            String::from("Error 501 - Not Implemented")),
        Route::from(
            String::from("/error-502/"),
            Vec::from([Method::Get]),
            String::from("Error 502 - Bad Gateway")),
        Route::from(
            String::from("/error-503/"),
            Vec::from([Method::Get]),
            String::from("Error 503 - Service Unavailable")),
        Route::from(
            String::from("/error-505/"),
            Vec::from([Method::Get]),
            String::from("Error 505 - HTTP Version Not Supported")),
        Route::from(
            String::from("/error-511"),
            Vec::from([Method::Get]),
            String::from("Error 511 - Network Authentication Required")),
    ];

    Ok(Json(routes))
}
