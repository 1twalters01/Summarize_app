use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};


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


