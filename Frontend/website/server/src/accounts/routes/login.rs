use actix_web::{get, web::Bytes, HttpResponse, Responder, Result};
use std::{fs, path::PathBuf};

#[get("/918.bundle.js")]
pub async fn login() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/918.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/374.bundle.js")]
pub async fn login_email() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/374.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/319.bundle.js")]
pub async fn login_password() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/319.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/37.bundle.js")]
pub async fn login_totp() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/575.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

