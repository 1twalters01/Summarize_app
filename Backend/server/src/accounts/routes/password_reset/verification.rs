use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};

use crate::{
    generated::protos::accounts::password_reset::verification::{
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


pub async fn post_verify(req_body: Json<VerificationRequest>, req: HttpRequest) -> Result<impl Responder> {
    let VerificationRequest { verification_token } = req_body.into_inner();
    let password_reset_email_token: String = req
        .headers()
        .get("password_reset_email_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    password_reset_verification_functionality(password_reset_email_token, verification_token).await
}

pub async fn link_verify(path: actix_web::web::Path<VerificationRequestSchema>) -> Result<impl Responder> {
    let VerificationRequestSchema {
        header_token,
        verification_token,
    } = path.into_inner();

    password_reset_verification_functionality(header_token, verification_token).await
}

async fn password_reset_verification_functionality(
    header_token: String,
    verification_token: String,
) -> Result<impl Responder> {
    let mut res_body: VerificationResponseSchema = VerificationResponseSchema::new();

    // Validate tokens

    // Form RegisterToken struct
    let token_struct: DualVerificationToken = DualVerificationToken {
        verification_token,
        header_token,
    };
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let user_json: String = match get_user_json_from_token_struct_in_redis(con, &token_struct_json) {
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
        Ok(user_json) => user_json,
    };

    // create a new token
    let password_reset_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        con,
        &password_reset_verification_token,
        &user_json,
        &expiry_in_seconds,
    );
    if set_redis_result.is_err() {
        panic!("redis error, panic debug")
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &token_struct_json);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return ok
    res_body.password_reset_response_token = Some(password_reset_verification_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

