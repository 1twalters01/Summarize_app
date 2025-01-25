use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::biometrics::{
        request::Request,
        response::{response, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{database_connections::create_redis_client_connection, validations::validate_password},
};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Result};

pub async fn post_biometrics(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request { password } = req_body.0;

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(password_response::ResponseField::Error(
                PasswordError::InvalidCredentials as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(password_response::ResponseField::Error(
                    PasswordError::InvalidCredentials as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    let user: User = match user_result {
        Err(_) => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(password_response::ResponseField::Error(
                    PasswordError::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let response: PasswordResponse = PasswordResponse {
                    response_field: Some(password_response::ResponseField::Error(
                        PasswordError::InvalidCredentials as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        },
    };

    // authenticate password
    if user.check_password(&password).is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(password_response::ResponseField::Error(
                PasswordError::IncorrectPassword as i32,
            )),
        };
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    };

    // Generate token
    let token: String = generate_opaque_token_of_length(25);

    // Save key: token, value: {token, uuid/jwt} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let set_redis_result = cache_service.store_key_value(&token, &user_uuid_str, expiry_in_seconds);
    if set_redis_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeBiometrics(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // If biometrics is active for user
        // generate challenge
        // save challenge to a challenge_token
        // response_field = response::ResponseField::DualResponse(token, challenge_token) // Need better name
        else {
            response_field = response::ResponseField::Token(token)
        }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangeBiometrics(Response {
            response_field: Some(response_field),
        }),
        StatusCode::OK,
    ));
}

pub async fn post_confirmation(
    req_body: ProtoBuf<TotpRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
}
