use actix_web::{get, post, HttpResponse, Responder, Result, web::Json};
// use std::{fs, path::PathBuf};
use crate::settings::schema::{
    ChangeEmailRequestStruct, ChangeEmailResponseStruct,
    ChangeUsernameRequestStruct, ChangeUsernameResponseStruct,
    ChangePasswordRequestStruct, ChangePasswordResponseStruct,
    DeleteAccountRequestStruct, DeleteAccountResponseStruct,
    ToggleTotpRequestStruct, ToggleTotpResponseStruct, Theme,
    ChangeThemeRequestStruct, ChangeThemeResponseStruct,
};

#[post("change-username")]
async fn change_username(req_body: Json<ChangeUsernameRequestStruct>) -> Result<impl Responder> {
    let ChangeUsernameRequestStruct { username, password } = req_body.into_inner();
    let res_body: ChangeUsernameResponseStruct = ChangeUsernameResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-email")]
async fn change_email(req_body: Json<ChangeEmailRequestStruct>) -> Result<impl Responder> {
    let ChangeEmailRequestStruct { email, password } = req_body.into_inner();
    let res_body: ChangeEmailResponseStruct = ChangeEmailResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-password")]
async fn change_password(req_body: Json<ChangePasswordRequestStruct>) -> Result<impl Responder> {
    let ChangePasswordRequestStruct { new_password, new_password_confirmation, password } = req_body.into_inner();
    let res_body: ChangePasswordResponseStruct = ChangePasswordResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("delete-account")]
async fn delete_account(req_body: Json<DeleteAccountRequestStruct>) -> Result<impl Responder> {
    let DeleteAccountRequestStruct { password, password_confirmation } = req_body.into_inner();
    let res_body: DeleteAccountResponseStruct = DeleteAccountResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("toggle-totp")]
async fn toggle_totp(req_body: Json<ToggleTotpRequestStruct>) -> Result<impl Responder> {
    let ToggleTotpRequestStruct { password, totp } = req_body.into_inner();
    let res_body: ToggleTotpResponseStruct = ToggleTotpResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-theme")]
async fn change_theme(req_body: Json<ChangeThemeRequestStruct>) -> Result<impl Responder> {
    let ChangeThemeRequestStruct { theme } = req_body.into_inner();
    let res_body: ChangeThemeResponseStruct = ChangeThemeResponseStruct::new();

    Ok(Json(res_body))
}

#[get("get-theme")]
async fn get_theme(req_body: Json<Theme>) -> Result<impl Responder> {
    let theme: Theme = Theme::Dark;

    Ok(Json(theme))
}



