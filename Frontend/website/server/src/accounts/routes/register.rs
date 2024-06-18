use actix_web::{get, web::Bytes, HttpResponse, Responder, Result};
use std::{fs, path::PathBuf};

#[get("/950.bundle.js")]
async fn register() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/950.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/302.bundle.js")]
pub async fn register_email() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/302.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/593.bundle.js")]
pub async fn register_verification() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/593.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/116.bundle.js")]
pub async fn register_verification_link() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/116.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/920.bundle.js")]
pub async fn register_details() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/920.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

