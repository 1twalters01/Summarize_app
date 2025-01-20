use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use uuid::Uuid;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::login::refresh::response::{
        response::ResponseField, Error, Response,
    },
    services::{response_service::ResponseService, token_service::TokenService},
    utils::validations::refresh_token::validate_refresh_token,
};

pub async fn post_refresh_token(req: HttpRequest) -> Result<impl Responder> {
    // Read refresh token from header if none then error
    let refresh_token: String = match req.headers().get("Refresh-Token") {
        Some(token) => match token.to_str() {
            Ok(refresh_token) => refresh_token.to_string(),
            Err(err) => {
                println!("err: {}", err);
                return Ok(ResponseService::create_error_response(
                    AppError::LoginRefresh(Error::InvalidRefreshToken),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
        },
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginRefresh(Error::RefreshTokenNotFound),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    // Validate refresh token
    if validate_refresh_token(&refresh_token).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginRefresh(Error::InvalidRefreshToken),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // try to Get user uuid from refresh token else fail
    let user_uuid: Uuid = match TokenService::get_user_uuid_from_refresh_token(&refresh_token).await
    {
        Ok(Some(user_uuid)) => user_uuid,
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginRefresh(Error::UserNotFound),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("err: {}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginRefresh(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    // Create new access token for user
    let token_service = TokenService::from_uuid(&user_uuid);
    let access_token: String = token_service.generate_access_token().unwrap();

    // Return token
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginRefresh(Response {
            response_field: Some(ResponseField::Token(access_token)),
        }),
        StatusCode::OK,
    ));
}
