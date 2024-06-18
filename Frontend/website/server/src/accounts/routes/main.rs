use actix_web::{get, web::{Bytes, Json}, HttpResponse, Responder, Result};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};

pub async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 8] = [
        Route::from(
            String::from("/login/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Log in to Summarize")),
        Route::from(
            String::from("/login/totp/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Authenticate user")),
        Route::from(
            String::from("/logout/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Log out of Summarize")),
        Route::from(
            String::from("/register/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Registers a new, unactivated user")),
        Route::from(
            String::from("/activate/{uidb64}/{token}/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Activate a newly registered user")),
        Route::from(
            String::from("/username-reset/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Reset the user's username")),
        Route::from(
            String::from("/password-reset/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Send a password reset email")),
        Route::from(
            String::from("/password-reset/{uidb64}/{token}/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Reset a password for a user")),
    ]; 

    Ok(Json(routes))
}


pub async fn main_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/main.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/175.bundle.js")]
pub async fn navbar() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/175.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

