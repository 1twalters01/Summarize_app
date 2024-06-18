use actix_web::{get, web::{Bytes, Json}, HttpResponse, Responder, Result};
use std::{fs, path::PathBuf};

#[get("/918.bundle.js")]
pub async fn login() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/918.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

pub async fn login_totp() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/575.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

pub async fn login_totp_post(req_body: actix_web::HttpRequest) -> Result<impl Responder> {
    println!("{:?}", req_body);

    Ok(Json(true))
}

