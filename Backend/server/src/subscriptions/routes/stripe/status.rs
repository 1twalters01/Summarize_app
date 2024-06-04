use actix_web::{HttpResponse, Responder, Result};


pub async fn retrieve_status() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub async fn new_customer() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub async fn success() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

