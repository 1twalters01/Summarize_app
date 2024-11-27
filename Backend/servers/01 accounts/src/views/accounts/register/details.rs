use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::register::details::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    models::user::User,
    queries::{
        postgres::user::insert::from_user,
        redis::{all::get_email_from_token_struct_in_redis, general::delete_key_in_redis},
    },
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::{
            validate_first_name, validate_last_name, validate_password, validate_username,
        },
    },
};

pub async fn post_details(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let Request {
        username,
        password,
        password_confirmation,
        first_name,
        last_name,
    } = data.0;

    let verification_confirmation_token: String = req
        .headers()
        .get("Register-Verification-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // get the email from redis using the token
    let mut con = create_redis_client_connection();
    let email: String =
        match get_email_from_token_struct_in_redis(&mut con, &verification_confirmation_token) {
            // if error return error
            Err(err) => {
                println!("error: {:#?}", err);
                return Ok(ResponseService::create_error_response(
                    AppError::RegisterDetails(Error::InvalidCredentials),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
            Ok(email) => email,
        };

    if password != password_confirmation {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::IncorrectPasswordConfirmation),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // check if the username is already found in the database. If it is then return error
    if validate_username(&username).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::InvalidUsername),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if validate_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::InvalidPassword),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if let Some(ref fname) = first_name {
        if validate_first_name(&fname).is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::RegisterDetails(Error::InvalidFirstName),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }

    if last_name.is_some() {
        if validate_last_name(last_name.clone().unwrap()).is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::RegisterDetails(Error::InvalidLastName),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }

    let create_user: Result<User, std::io::Error> =
        User::new(username, email, password, first_name, last_name);
    if create_user.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let user: User = create_user.unwrap();

    // save details to the user to postgres
    let pool = create_pg_pool_connection().await;
    let save_user_result: Result<(), sqlx::Error> = from_user(&pool, user).await;

    // if error then return error
    if save_user_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(&mut con, &verification_confirmation_token);

    // if redis fails then return an error
    if delete_redis_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // set created time

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::RegisterDetails(Response {
            response_field: Some(ResponseField::Success(Success {})),
        }),
        StatusCode::OK,
    ));
}

#[cfg(test)]
mod tests {
    // use actix_web::{test, web, App};
    // use dotenv::dotenv;

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_without_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_token_with_header_token_username_password_without_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_without_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_first_without_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_without_header_token_username_password_confirmation_with_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_without_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_token_with_header_token_username_password_without_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_without_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_first_without_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_without_header_token_username_password_confirmation_with_first_last(
    ) {
    }
}
