use actix_web::{get,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};

#[get("/download/")]
async fn download_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/ownload.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/download.js")]
async fn download_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/download/web-clipper/")]
async fn download_web_clipper_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download-web-clipper.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
      .content_type("text/html; charset=UTF-8")
      .body(data))
}

#[get("/download/web-clipper.js")]
async fn download_web_clipper_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download-web-clipper.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
      .content_type("text/javascript; charset=UTF-8")
      .body(data))
}

#[get("/download/mobile/")]
async fn download_mobile_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download-mobile.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/download/mobile.js")]
async fn download_mobile_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download-mobile.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/download/desktop/")]
async fn download_desktop_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download-desktop.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/download/desktop.js")]
async fn download_desktop_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/download-desktop.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

