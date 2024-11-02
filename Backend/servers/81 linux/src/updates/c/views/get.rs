use actix_web::{HttpResponse, Responder, Result, web::Bytes};
use std::{fs, path::PathBuf, env};

pub async fn get_all() -> Result<impl Responder> {
    // send all c files in a response
}
