use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use ring::signature;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    models::user::User,
    generated::protos::accounts::{
        auth_tokens::AuthTokens,
        login::biometrics::{
            request,
            response::{response::ResponseField, Error, Response},
        },
    },
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{database_connections::create_redis_client_connection, validations::validate_totp},
}

pub async fn post_biometrics(
    data: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let login_biometrics_token: String = req
        .headers()
        .get("Login-Biometrics-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let challenge_token: String = req
    .headers()
    .get("Challenge-Token")
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();

    // get user uuid
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_uuid_from_token(&token_tuple_json);
    let user_uuid: Uuid = match cache_result {
        Ok(Some(uuid)) => uuid,
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginBiometrics(Error::ServerError),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetVerification(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };
    
    // Get challenge and device_id from token
    let cache_result = cache_service.get_challenge_from_token(&login_biometrics_token);
    let (challenge, device_id) = match cache_result {
        Ok(Some((challenge, device_id))) => (challenge, device_id),
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginBiometrics(Error::ServerError),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetVerification(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    let Request {
        device_id,
        platform_id,
        encoded_signed_challenge,
    } = data.0;

    let signed_challenge = match base64::decode(&encoded_signed_challenge) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetVerification(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    // get public key for the device id and the user uuid else error
    let user_service  = UserService::new(create_pg_pool_connection().await);
    let public_key_pem = user_service.get_biometrics_public_key_from_uuid_and_device_id(&user_uuid, &device_id)
    
    let public_key = signature::UnparsedPublicKey::new(
        &ring::signature::ECDSA_P256_SHA256_ASN1,
        public_key_pem.as_bytes(),
    );
    if public_key.verify(challenge.as_bytes(), &signed_challenge).is_err() {
        return HttpResponse::Unauthorized().body("Invalid signature");
    }
    
    // create auth tokens
    let token_service = TokenService::from_uuid(&user_uuid);
    let refresh_token = token_service.generate_refresh_token();
    let access_token = token_service.generate_access_token().unwrap();

    let save_result = token_service
        .save_refresh_token_to_postgres(&refresh_token, remember_me)
        .await;
    if save_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // update last login time
    let pool = create_pg_pool_connection().await;
    if update_login_time_from_uuid(&pool, chrono::Utc::now(), &user_uuid)
        .await
        .is_err()
    {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    };

    // generate opaque token with prefix SITE_
    // save: con.set_ex(format!("session:{}", opaque_token), access_token, expiration as usize)

    // delete old tokens
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.delete_key(&login_biometrics_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginSms(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }
    let cache_result = cache_service.delete_key(&challenge_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginSms(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return opaque token to user
    let auth_tokens = AuthTokens {
        refresh: refresh_token,
        access: access_token,
    };
    println!("auth tokens: {:#?}", auth_tokens);
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginBiometrics(Response {
            response_field: Some(ResponseField::Tokens(auth_tokens)),
        }),
        StatusCode::OK,
    ));
}
