use crate::accounts::schema::auth::Claims;
use crate::accounts::datatypes::users::User;
use crate::settings::schema::{
    SettingsError, ToggleTotpRequestStruct, ToggleTotpResponseStruct
};
use crate::utils::validations::{
    validate_password, validate_totp
};
use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};

#[post("toggle-totp")]
async fn toggle_totp(
    req_body: Json<ToggleTotpRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ToggleTotpRequestStruct { password, totp } = req_body.into_inner();
    let mut res_body: ToggleTotpResponseStruct = ToggleTotpResponseStruct::new();

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
    let validated_email = validate_totp(&totp);
    if validated_email.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_email.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // if the entered totp doesn't match the generated code then error

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // authenticate password

    // remove totp
    // if error on removal then return error

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

