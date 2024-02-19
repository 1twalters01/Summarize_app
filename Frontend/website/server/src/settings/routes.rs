use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};

#[get("/get-routes")]
async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 6] = [
        Route::from(
            String::from("/change-email/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Change the user's email")),
        Route::from(
            String::from("/change-password/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Change the user's password")),
        Route::from(
            String::from("/change-username/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Change the user's username")),
        Route::from(
            String::from("/delete-account/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Delete the user's account")),
        Route::from(
            String::from("/theme/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Change the user's theme")),
        Route::from(
            String::from("/two-factor-authentication/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Two factor authentication")),
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

#[get("/942.bundle.js")]
async fn change_username() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/942.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/788.bundle.js")]
async fn change_email() -> Result<impl Responder> {
    println!("change email");
    let path: PathBuf = "../content/dist/main/javascript/788.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/221.bundle.js")]
async fn change_password() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/221.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/15.bundle.js")]
async fn change_theme() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/15.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/981.bundle.js")]
async fn two_factor_auth() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/981.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/104.bundle.js")]
async fn close_account() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/104.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}


