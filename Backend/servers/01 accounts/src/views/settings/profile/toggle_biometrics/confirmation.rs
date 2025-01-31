use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::confirmation::{
        response, Error, Request as PasswordRequest,
        Response , Success,
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::password::validate_password,
    },
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_confirmation(
    req_body: ProtoBuf<PasswordRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request {
        device_id,
        platform_type,
        enum {
            encoded_signed_challenge,
            public_key,
        }
    } = req_body.0;
    let toggle_biometrics_token: String = req
        .headers()
        .get("Toggle-Biometrics-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // validate device id and then public key or encoded_signed_challenge
    let validated_device_id = validate_device_id(&device_id);
    if validated_device_id.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::Confirmation(Error::InvalidCredentials),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }
    if let Some(public_key) = public_key {
        let validated_public_key = validate_public_key(&public_key);
        if validated_public_key.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }
    if let Some() {
        let signed_challenge = match base64::decode(&encoded_signed_challenge) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Ok(ResponseService::create_error_response(
                    AppError::PasswordResetVerification(Error::InvalidCredentials),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
        };
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
        },
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

    // get current biometrics status
    let user_service = UserService::new(create_pg_pool_connection().await);
    let get_result = user_service.get_biometrics_activation_status_from_uuid(&user_uuid).await;
    let biometrics_activation_status = match get_result {
        Ok(result) => result,
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    // if public key = there then delete
    if biometrics_activation_status == true {
        let get_result: Result<Option<String>, sqlx::Error> =
            user_service.get_biometrics_public_key_from_uuid(&user_uuid).await;
        let public_key_pem: Option<String> = match get_result {
            Ok(result) => result,
            Err(_) => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };

        let public_key = ring::signature::UnparsedPublicKey::new(
            &ring::signature::ECDSA_P256_SHA256_ASN1,
            public_key_pem.as_bytes(),
        );
        if public_key.verify(challenge.as_bytes(), &signed_challenge).is_err() {
            return HttpResponse::Unauthorized().body("Invalid signature");
        }

        let delete_result = user_service.delete_biometrics_from_uuid(&user_uuid).await;
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
        let public_key_pem = cache_service.get_biometrics_key_from_user_uuid_and_device_id(&user_uuid, &device_id)
    
        let public_key = signature::UnparsedPublicKey::new(
            &ring::signature::ECDSA_P256_SHA256_ASN1,
            public_key_pem.as_bytes(),
        );
        if public_key.verify(challenge.as_bytes(), &signed_challenge).is_err() {
            return HttpResponse::Unauthorized().body("Invalid signature");
        }

        let set_result = user_service.set_biometrics_from_uuid(&public_key_pem, device_id, platform_type, &user_uuid).await;
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
