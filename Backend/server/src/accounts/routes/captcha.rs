use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::{
    accounts::{db_queries::get_code_from_token_in_redis, schema::AccountError},
    utils::{
        database_connections::{create_redis_client_connection, set_key_value_in_redis},
        tokens::generate_opaque_token_of_length,
    },
};

#[derive(Serialize)]
struct GetCaptchaResponseSchema {
    account_error: AccountError,
    success: bool,
}

impl GetCaptchaResponseSchema {
    pub fn new() -> GetCaptchaResponseSchema {
        GetCaptchaResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}

#[get("/captcha")]
async fn get_captcha() -> Result<impl Responder> {
    let mut res_body: GetCaptchaResponseSchema = GetCaptchaResponseSchema::new();

    // generate captcha

    // get answer for captcha
    let answer = String::from("captcha answer");

    // generate 64 bit token
    let token = generate_opaque_token_of_length(64);

    // save { key: token, value: answer } to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token, &answer, &expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.await.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // send image and token
    return Ok(HttpResponse::Unauthorized()
        .content_type("application/json; charset=utf-8")
        .json(true));
}

#[derive(Deserialize)]
struct CaptchaResponse {
    token: String,
    response: String,
}

#[derive(Serialize)]
struct CaptchaResponseSchema {
    account_error: AccountError,
    success: bool,
}

impl CaptchaResponseSchema {
    pub fn new() -> CaptchaResponseSchema {
        CaptchaResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}

#[post("/verify_captcha")]
async fn verify_captcha(data: web::Json<CaptchaResponse>) -> Result<impl Responder> {
    let CaptchaResponse { token, response } = data.into_inner();
    let mut res_body: CaptchaResponseSchema = CaptchaResponseSchema::new();

    // Retrieve the solution from the session or database
    let con = create_redis_client_connection();
    let solution: String = match get_code_from_token_in_redis(con, &token) {
        // if error return error
        Err(err) => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(err),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Ok(solution) => solution,
    };

    if response == solution {
        res_body.success = true;
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    } else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
}
