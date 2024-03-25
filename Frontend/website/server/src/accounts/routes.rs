use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
// use users::credentials::Credentials;
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};
// use users::user::User;

#[get("/get-routes/")]
async fn get_routes() -> Result<impl Responder> {
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

#[get("/{param:.*?}")]
async fn main_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/main.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/104.bundle.js")]
async fn login() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/104.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/575.bundle.js")]
async fn login_totp() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/575.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[post("/login/2fa")]
async fn login_totp_post(req_body: actix_web::HttpRequest) -> Result<impl Responder> {
    println!("{:?}", req_body);

    Ok(Json(true))
}

#[get("/837.bundle.js")]
async fn logout() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/837.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/116.bundle.js")]
async fn register() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/116.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/452.bundle.js")]
async fn activate() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/116.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/18.bundle.js")]
async fn username_reset() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/18.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/615.bundle.js")]
async fn username_reset_token() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/615.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/767.bundle.js")]
async fn password_reset() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/767.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/407.bundle.js")]
async fn password_reset_token() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/407.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

