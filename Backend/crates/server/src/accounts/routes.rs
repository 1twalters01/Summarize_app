use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};

#[post("change-email")]
async fn change_email(req_body: web::Json<EmailPassword>) -> Result<impl Responder> {
    let res_body: Payload2 = Payload2 {
        email: param.into_inner().email,
        password: param.into_inner().password,
    };

    Ok(web::Json(res_body))
}
