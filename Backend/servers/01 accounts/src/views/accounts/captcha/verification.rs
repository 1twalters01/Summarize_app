use actix_web::{web, HttpResponse, Responder, Result};
use captcha::{
    filters::{Dots, Noise},
    Captcha,
};
use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::captcha::verification::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    services::{
        cache_service::CacheService,
        responose_service::ResponseService,
    },
}

pub async fn verify_captcha(data: web::Json<CaptchaResponse>) -> Result<impl Responder> {
    let CaptchaResponse { token, response } = data.into_inner();
    let mut res_body: CaptchaResponseSchema = CaptchaResponseSchema::new();

    // Retrieve the solution from the session or database
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result =
        cache_service.get_answer_from_token(&token);
    let solution = match cache_result {
        Ok(solution) => solution,
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
    }

    if response == solution {
        // Save that captcha was successful for given ip for x mins
        // Or just create a jwt (or something else) with an expiry date
        return Ok(ResponseService::create_success_response(
            AppResponse::CaptchaVerification(Response {
                response_field: Some(ResponseField::Success(Success {})),
            }),
            StatusCode::OK,
        ));
    } else {
        return Ok(ResponseService::create_error_response(
            AppError::CaptchaGet(Error::IncorrectCaptcha),
            StatusCode::UNAUTHORIZED,
        ));
    }
}