use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::confirmation::{
        request::TotpRequest as Request,
        response::{response, Error, Response, Success},
    },
    models::{totp::Totp, user::User},
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::totp::validate_totp,
    },
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};
use uuid::Uuid;

pub async fn post_confirmation(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request {
        digit1,
        digit2,
        digit3,
        digit4,
        digit5,
        digit6,
    } = req_body.0;
    let toggle_totp_token: String = req
        .headers()
        .get("Toggle-Totp-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // validate totp digits
    let digits = &[digit1, digit2, digit3, digit4, digit5, digit6];
    let validated_totp = validate_totp(digits);
    if validated_totp.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::Confirmation(Error::InvalidCredentials),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid_str: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid_str).await;
    let user: User = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // get uuid from redis - make the jwt the token instead of uuid for safety?
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let result = cache_service.get_user_uuid_from_token(&toggle_totp_token);
    let saved_uuid: Uuid = match result {
        // if error return error
        Err(err) => {
            println!("err: {:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(uuid) => match uuid {
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
            Some(uuid) => uuid,
        },
    };

    // if saved_uuid != uuid then error
    if user_uuid_str != saved_uuid.to_string() {
        return Ok(ResponseService::create_error_response(
            AppError::Confirmation(Error::InvalidCredentials),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    let user_uuid = user.get_uuid();

    // get current totp status
    let user_service = UserService::new(create_pg_pool_connection().await);
    let get_result = user_service.get_totp_activation_status_from_uuid(&user_uuid).await;
    let totp_activation_status = match get_result {
        Ok(result) => result,
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    
    // if totp = activated then validate then delete delete
    if totp_activation_status {
        let get_result: Result<Option<String>, sqlx::Error> =
            user_service.get_totp_key_from_uuid(&user_uuid).await;
        let totp_key: Option<String> = match get_result {
            Ok(result) => result,
            Err(_) => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };
        let mut totp: Totp = Totp::from_key(totp_key);

        if totp
            .verify(digit1, digit2, digit3, digit4, digit5, digit6)
            .is_err()
        {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::IncorrectTotp),
                StatusCode::UNAUTHORIZED,
            ));
        };

        let delete_result = user_service.delete_totp_from_uuid(&user_uuid).await;
        match delete_result {
            Ok(_) => {
                return Ok(ResponseService::create_success_response(
                    AppResponse::Confirmation(Response {
                        response_field: Some(response::ResponseField::Success(Success {})),
                    }),
                    StatusCode::OK,
                ));
            }
            Err(_) => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }
    } else {
        let cache_result = cache_service.get_totp_key_from_uuid(user_uuid);
        let totp_key: Option<String> = match cache_result {
            Ok(Some(result)) => result,
            Ok(None) => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::NOT_FOUND,
                ));
            }
            Err(_) => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };
        let totp: Totp = Totp::from_key(totp_key);
        if totp
            .verify(digit1, digit2, digit3, digit4, digit5, digit6)
            .is_err()
        {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::IncorrectTotp),
                StatusCode::UNAUTHORIZED,
            ));
        };
        let set_result = user_service.set_totp_from_uuid(&totp, &user_uuid).await;
        match set_result {
            Ok(_) => {
                return Ok(ResponseService::create_success_response(
                    AppResponse::Confirmation(Response {
                        response_field: Some(response::ResponseField::Success(Success {})),
                    }),
                    StatusCode::OK,
                ));
            }
            Err(_) => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }
    }
}
