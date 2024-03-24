use actix_web::{post, HttpResponse, Responder, Result, web::Json};
use crate::accounts::datatypes::Login;
// use std::{fs, path::PathBuf};

#[post("login")]
async fn login(req_body: Json<Login>) -> Result<impl Responder> {
    let res_body: Login = Login {
        email: req_body.clone().email,
        password: req_body.clone().password,
        totp: req_body.into_inner().totp,
    };

    println!("res body: {:#?}", res_body);

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

