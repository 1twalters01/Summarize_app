use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};

#[get("/get-routes")]
async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 7] = [
        Route::from(
            String::from("/error-300/"),
            Vec::from([Method::Get]),
            String::from("Error 300 - Multiple Choices")),
        Route::from(
            String::from("/error-301/"),
            Vec::from([Method::Get]),
            String::from("Error 301 - Moved Permanently")),
        Route::from(
            String::from("/error-303/"),
            Vec::from([Method::Get]),
            String::from("Error 303 - See Other")),
        Route::from(
            String::from("/error-304/"),
            Vec::from([Method::Get]),
            String::from("Error 304 - Not Modified")),
        Route::from(
            String::from("/error-307/"),
            Vec::from([Method::Get]),
            String::from("Error 307 - Temporary Redirect")),
        Route::from(
            String::from("/error-308"),
            Vec::from([Method::Get]),
            String::from("Error 308 - Permanent Redirect")),
    ];

    Ok(Json(routes))
}
