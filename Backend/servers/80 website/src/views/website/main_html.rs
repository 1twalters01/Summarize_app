use actix_web::{HttpResponse, Responder, Result, web::Bytes};
use std::{fs, path::PathBuf, env};

pub async fn main_html() -> Result<impl Responder> {
    let website_dir: String = env::var("WEBSITE_DIR").unwrap();
    let path: PathBuf = format!("{}/dist/main/index.html", website_dir).into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}