use actix_web::{post, HttpResponse, Responder, Result, web::Json};
use crate::accounts::datatypes::{LoginEmail, LoginPassword, LoginTotp};
// use std::{fs, path::PathBuf};

#[post("login/email")]
async fn login_email(req_body: Json<LoginEmail>) -> Result<impl Responder> {
    let res_body: String = req_body.into_inner().email;

    println!("res body: {:#?}", res_body);

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("login/password")]
async fn login_password(req_body: Json<LoginPassword>) -> Result<impl Responder> {
    let res_body: LoginPassword = LoginPassword {
        email: req_body.clone().email,
        password: req_body.into_inner().password,
    };

    println!("res body: {:#?}", res_body);

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}
#[post("login/totp")]
async fn login_totp(req_body: Json<LoginTotp>) -> Result<impl Responder> {
    let res_body: LoginTotp = LoginTotp {
        email: req_body.clone().email,
        password: req_body.clone().password,
        totp: req_body.into_inner().totp,
    };

    println!("res body: {:#?}", res_body);

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

