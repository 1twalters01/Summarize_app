use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};

use crate::{
    generated::protos::accounts::register::details::{
        request,
        response::{self, response::ResponseField},
    },
    models::user::User,
    queries::{
        postgres::user::insert::from_user,
        redis::{all::get_email_from_token_struct_in_redis, general::delete_key_in_redis},
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::{
            validate_first_name, validate_last_name, validate_password, validate_username,
        },
    },
};

pub async fn post_details(
    data: ProtoBuf<request::Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let request::Request {
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
                let response: response::Response = response::Response {
                    response_field: Some(ResponseField::Error(
                        response::Error::InvalidCredentials as i32,
                    )),
                };

                return Ok(HttpResponse::UnprocessableEntity()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
            Ok(email) => email,
        };

    if password != password_confirmation {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::IncorrectPasswordConfirmation as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // check if the username is already found in the database. If it is then return error
    let validated_username = validate_username(&username);
    if validated_username.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::InvalidUsername as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::InvalidPassword as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    if first_name.is_some() {
        let validated_first_name = validate_first_name(first_name.clone().unwrap());
        if validated_first_name.is_err() {
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(
                    response::Error::InvalidFirstName as i32,
                )),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    }

    if last_name.is_some() {
        let validated_last_name = validate_last_name(last_name.clone().unwrap());
        if validated_last_name.is_err() {
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(
                    response::Error::InvalidLastName as i32,
                )),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    }

    let create_user: Result<User, std::io::Error> =
        User::new(username, email, password, first_name, last_name);
    if create_user.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let user: User = create_user.unwrap();

    // save details to the user to postgres
    let pool = create_pg_pool_connection().await;
    let save_user_result: Result<(), sqlx::Error> = from_user(&pool, user).await;

    // if error then return error
    if save_user_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &verification_confirmation_token);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return Ok
    // create an auth token with remember me set to false and send it over as well?
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Success(response::Success {})),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
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
