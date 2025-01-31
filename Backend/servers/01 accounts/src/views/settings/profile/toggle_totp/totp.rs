use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::totp::{
        request::Request,
        response::{response, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{
        database_connections::create_redis_client_connection,
        validations::password::validate_password,
    },
};
use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_totp(req_body: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let Request { password } = req_body.0;

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeTotp(Error::InvalidCredentials),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid_str: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeTotp(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid_str).await;
    let user: User = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeTotp(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::ChangeTotp(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // authenticate password
    if user.check_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeTotp(Error::IncorrectPassword),
            StatusCode::UNAUTHORIZED,
        ));
    };

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);

    // Save key: token, value: {token, uuid/jwt} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let set_redis_result = cache_service.store_key_value(&token, &user_uuid_str, expiry_in_seconds);
    if set_redis_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeTotp(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    let user_service = UserService::new(create_pg_pool_connection().await);
    let get_result: Result<Option<String>, sqlx::Error> =
        user_service.get_totp_activation_status_from_uuid(&user_uuid).await;
    let totp_activation_status = match get_result {
        Ok(result) => result,
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    if totp_activation_status == false {
        let totp_key = token_service.generate_opaque_token_of_length(25);

        let set_redis_result = cache_service.store_totp_key_for_user_uuid(
            &format("totp_key for:{}", user.get_uuid()),
            &totp_key,
            expiry_in_seconds,
        );
        if set_redis_result.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeTotp(Error::ServerError),
                StatusCode::FAILED_DEPENDENCY,
            ));
        }
        let totp = Totp::from_key(totp_key);
        let qr_string = totp.generate_qr_string();
        response_field = response::ResponseField::CreateResponse(token, qr_string)
    else {
        response_field = response::ResponseField::DeleteResponse(token)
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangeTotp(Response {
            response_field: Some(response_field),
        }),
        StatusCode::OK,
    ));
}
