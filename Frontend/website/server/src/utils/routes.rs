use actix_web::{HttpResponse, Responder, Result, web::Bytes};
use std::{fs, path::PathBuf};

pub async fn main_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/index.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

pub async fn main_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}


