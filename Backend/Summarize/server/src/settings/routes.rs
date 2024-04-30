use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder, Result};
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
    async fn change_username(req_body: Json<ChangeUsernameRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let ChangeUsernameRequestStruct { username, password } = req_body.into_inner();
    let res_body: ChangeUsernameResponseStruct = ChangeUsernameResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // validate password
    let validated_password = validate_password(password.clone());
    if validated_password.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // validate username
    let validated_username = validate_username(username.clone());
    if validated_username.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_username.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // error if username is already taken


    // authenticate password


    // change username

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-email")]
async fn change_email(req_body: Json<ChangeEmailRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let ChangeEmailRequestStruct { email, password } = req_body.into_inner();
    let res_body: ChangeEmailResponseStruct = ChangeEmailResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // validate password
    let validated_password = validate_password(password.clone());
    if validated_password.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // validate email
    let validated_email = validate_email(email.clone());
    if validated_email.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // error if email is already taken


    // authenticate password


    // change email


    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-password")]
async fn change_password(req_body: Json<ChangePasswordRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let ChangePasswordRequestStruct { new_password, new_password_confirmation, password } = req_body.into_inner();
    let res_body: ChangePasswordResponseStruct = ChangePasswordResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // error if new_password != new_password_confirmation
    if new_password != new_password_confirmation {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(String("confirmation is not the same as the new password"))
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // validate password
    let validated_password = validate_password(password.clone());
    if validated_password.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // validate new password
    let validated_new_password = validate_password(new_password.clone());
    if validated_new_password.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_new_password.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // authenticate password


    // change password 


    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("delete-account")]
async fn delete_account(req_body: Json<DeleteAccountRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let DeleteAccountRequestStruct { password, password_confirmation } = req_body.into_inner();
    let res_body: DeleteAccountResponseStruct = DeleteAccountResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("toggle-totp")]
async fn toggle_totp(req_body: Json<ToggleTotpRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let ToggleTotpRequestStruct { password, totp } = req_body.into_inner();
    let res_body: ToggleTotpResponseStruct = ToggleTotpResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // check if user has totp enabled
    // if no then:
        // if totp is not none then return error
        // generate a totp string
        // set totp
        // if error on setting totp then return error

    // get user's totp string
    // get totp code from string

    // validate the entered totp code
    let validated_email = validate_totp(totp.clone());
    if validated_email.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    // if the entered totp doesn't match the generated code then error
    
    // validate password
    let validated_password = validate_password(password.clone());
    if validated_password.is_error() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }


    // authenticate password 


    // remove totp
    // if error on removal then return error


    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[get("get-theme")]
async fn get_theme(req: HttpRequest) -> Result<impl Responder> {
    let res_body: GetThemeResponseStruct = GetThemeResponseStruct::new();
    // get user's device - from header? - this means no req_body
    // get user's theme for the device
    // if error when getting the user's theme then return error

    // return ok


    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-theme")]
async fn change_theme(req_body: Json<ChangeThemeRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let ChangeThemeRequestStruct { theme } = req_body.into_inner();
    let res_body: ChangeThemeResponseStruct = ChangeThemeResponseStruct::new();

    // get user's device (linux app, windows app, mac app, android app, ios app, desktop website, mobile website)
    // set user's theme for the current device to what was entered
    // if setting error then return error

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("change-language")]
async fn change_language(req_body: Json<ChangeLanguageRequestStruct>, req: HttpRequest) -> Result<impl Responder> {
    let ChangeLanguageRequestStruct { language } = req_body.into_inner();
    let res_body: ChangeLanguageResponseStruct = ChangeLanguageREsponseStruct::new();

    // validate the language
    // update the user's language to the new one
    // if error when updating then error
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}


