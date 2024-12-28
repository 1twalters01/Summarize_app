use actix_web::{web, HttpResponse, Responder, Result};
use captcha::{
    filters::{Dots, Noise},
    Captcha,
};

use crate::{
    queries::redis::{
        all::get_code_from_token_in_redis,
        general::set_key_value_in_redis,
    },
    accounts::schema::captcha::{AccountError, CaptchaResponse, CaptchaResponseSchema, GetCaptchaResponseSchema},
    utils::{
        database_connections::create_redis_client_connection,
        tokens::generate_opaque_token_of_length,
    },
};

pub async fn get_captcha() -> Result<impl Responder> {
    let mut res_body: GetCaptchaResponseSchema = GetCaptchaResponseSchema::new();

    // generate captcha
    let mut captcha = Captcha::new();
    captcha
        .add_chars(6)
        .apply_filter(Noise::new(0.4))
        .apply_filter(Dots::new(10));

    let image_data = captcha.as_png().unwrap();

    // get answer for captcha
    let answer: String = captcha.chars_as_string();

    // generate 64 bit token
    let token = generate_opaque_token_of_length(64);

    // save { key: token, value: answer } to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token, &answer, expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // create body that will be used for multipart
    let mut body = Vec::new();
    let boundary = "BOUNDARY";

    // Add image part to body
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"image.png\"\r\n");
    body.extend_from_slice(b"Content-Type: image/png\r\n\r\n");
    body.extend_from_slice(&image_data);
    body.extend_from_slice(b"\r\n");

    // Add text part
    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"text\"\r\n");
    body.extend_from_slice(b"Content-Type: text/plain\r\n\r\n");
    body.extend_from_slice(token.as_bytes());
    body.extend_from_slice(b"\r\n");

    // End the multipart body
    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(body));
}

