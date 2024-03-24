use actix_web::{get, post, HttpResponse, Responder, Result, web::Json};
// use std::{fs, path::PathBuf};
use crate::settings::datatypes::{ChangeEmailStruct, ChangeThemeStruct, ChangeUsernameStruct, ChangePasswordStruct, ToggleTotpStruct, Theme, DeleteAccountStruct};

#[post("change-username")]
async fn change_username(req_body: Json<ChangeUsernameStruct>) -> Result<impl Responder> {
    let res_body: ChangeUsernameStruct = ChangeUsernameStruct {
        username: req_body.clone().username,
        password: req_body.into_inner().password,
    };

    Ok(Json(res_body))
}

#[post("change-email")]
async fn change_email(req_body: Json<ChangeEmailStruct>) -> Result<impl Responder> {
// async fn change_email(req_body: Json<ChangeEmailStruct>) -> Result<impl Responder> {
    let res_body: ChangeEmailStruct = ChangeEmailStruct {
        email: req_body.clone().email,
        password: req_body.into_inner().password,
    };

    // Ok(Json(res_body))
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .insert_header(("Authentication Token", ""))
        .json(res_body))
}

#[post("change-password")]
async fn change_password(req_body: Json<ChangePasswordStruct>) -> Result<impl Responder> {
    let res_body: ChangePasswordStruct = ChangePasswordStruct {
        password: req_body.clone().password,
        new_password: req_body.clone().new_password,
        new_password_confirmation: req_body.into_inner().new_password_confirmation
    };

    Ok(Json(res_body))
}

#[get("get-totp")]
async fn get_totp(req_body: Json<String>) -> Result<impl Responder> {
    let totp: String = String::new();
    Ok(Json(totp))
}

#[post("toggle-totp")]
async fn add_totp(req_body: Json<ToggleTotpStruct>) -> Result<impl Responder> {
    let res_body: ToggleTotpStruct = ToggleTotpStruct {
        totp: req_body.clone().totp,
        password: req_body.into_inner().password,
    };

    Ok(Json(res_body))
}

#[get("get-theme")]
async fn get_theme(req_body: Json<Theme>) -> Result<impl Responder> {
    let theme: Theme = Theme::Dark;

    Ok(Json(theme))
}

#[post("change-theme")]
async fn change_theme(req_body: Json<ChangeThemeStruct>) -> Result<impl Responder> {
    let res_body: ChangeThemeStruct = ChangeThemeStruct {
        theme: req_body.clone().theme,
        password: req_body.into_inner().password,
    };

    Ok(Json(res_body))
}

#[post("delete-account")]
async fn delete_account(req_body: Json<DeleteAccountStruct>) -> Result<impl Responder> {
    let res_body: DeleteAccountStruct = DeleteAccountStruct {
        password: req_body.clone().password,
        password_confirmation: req_body.into_inner().password_confirmation,
    };

    Ok(Json(res_body))
}

