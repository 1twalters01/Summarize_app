use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::captcha::verification::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    services::{
        cache_service::CacheService,
        response_service::ResponseService,
    }, utils::database_connections::create_redis_client_connection,
};

pub async fn verify_captcha(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let captcha_verification_token: String = req
        .headers()
        .get("Captcha-Verification-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let Request { input_solution } = data.0;

    // Retrieve the stored_solution from the session or database
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result =
        cache_service.get_answer_from_token(&captcha_verification_token);
    let stored_solution = match cache_result {
        Ok(stored_solution) => stored_solution,
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::CaptchaVerification(Error::IncorrectCaptcha),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    if input_solution == stored_solution {
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
            AppError::CaptchaVerification(Error::IncorrectCaptcha),
            StatusCode::UNAUTHORIZED,
        ));
    }
}
