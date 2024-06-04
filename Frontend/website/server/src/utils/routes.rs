use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
// use crate::datatypes::route::{Route, Method};


#[get("/504.bundle.js")]
async fn cookies() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/504.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

