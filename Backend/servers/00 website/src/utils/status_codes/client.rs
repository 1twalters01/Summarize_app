use actix_web::{get, Responder, Result, web::Json};
use crate::utils::status_codes::datatypes::{Route, Method};

#[get("/get-routes")]
async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 7] = [
        Route::from(
            String::from("/error-400/"),
            Vec::from([Method::Get]),
            String::from("Error 400 - Bad Request")),
        Route::from(
            String::from("/error-401/"),
            Vec::from([Method::Get]),
            String::from("Error 401 - Unauthorized")),
        Route::from(
            String::from("/error-403/"),
            Vec::from([Method::Get]),
            String::from("Error 403 - Forbidden")),
        Route::from(
            String::from("/error-404/"),
            Vec::from([Method::Get]),
            String::from("Error 404 - Not Found")),
        Route::from(
            String::from("/error-405/"),
            Vec::from([Method::Get]),
            String::from("Error 405 - Method Not Allowed")),
        Route::from(
            String::from("/error-410"),
            Vec::from([Method::Get]),
            String::from("Error 410 - Gone")),
        Route::from(
            String::from("/error-429/"),
            Vec::from([Method::Get]),
            String::from("Error 429 - Too Many Requests")),
    ];

    Ok(Json(routes))
}
