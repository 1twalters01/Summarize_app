use actix_web::{HttpResponse, Responder, Result, web::Bytes};
use std::{fs, path::PathBuf, env};

pub async fn main_js() -> Result<impl Responder> {
    let website_dir: String = env::var("WEBSITE_DIR").unwrap();
    let path: PathBuf = format!("{}/dist/main/javascript/bundle.js", website_dir).into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}