use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};

use crate::{
    generated::protos::accounts::password_reset::password::{
        response::{self, response::ResponseField},
        request,
    },
    accounts::{
        datatypes::users::User,
        queries::{
            postgres::{
                get_user_from_email_in_pg_users_table,
                update_password_for_user_in_pg_users_table,
            },
            redis::{
                get_user_json_from_token_struct_in_redis,
                get_user_from_token_in_redis,
            }
        },
        emails::compose_password_reset_email_message,
        schema::{
            errors::AccountError,
            password_reset::{
                PasswordResetConfirmRequestSchema,
                PasswordResetConfirmResponseSchema, PasswordResetRequestSchema,
                PasswordResetResponseSchema,
                DualVerificationToken,
                VerificationRequest, VerificationRequestSchema, VerificationResponseSchema,
            },
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
            set_key_value_in_redis,
        },
        email_handler::send_email,
        tokens::generate_opaque_token_of_length,
        validations::{validate_email, validate_password},
    },
};


pub async fn post_password_reset(
    req_body: Json<PasswordResetConfirmRequestSchema>,
    req: HttpRequest,
) -> Result<impl Responder> {
    println!("hi");
    let verification_confirmation_token: String = req
        .headers()
        .get("password_reset_verification_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    println!("verification token: {:?}", verification_confirmation_token);

    let PasswordResetConfirmRequestSchema {
        password,
        password_confirmation,
    } = req_body.into_inner();
    let mut res_body: PasswordResetConfirmResponseSchema =
        PasswordResetConfirmResponseSchema::new();

    if password != password_confirmation {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(false));
    }

    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()));
    }

    // get user from token in redis
    let con = create_redis_client_connection();
    let mut user: User = match get_user_from_token_in_redis(con, &verification_confirmation_token) {
        // if error return error
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
        Ok(email) => email,
    };

    // if change is not allowed then error
    let set_password_result = user.set_password(password);
    if set_password_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("unable to set password")),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // save change in postgres
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_password_for_user_in_pg_users_table(&pool, &user).await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    println!("complete");

    // return success
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(true));
}
