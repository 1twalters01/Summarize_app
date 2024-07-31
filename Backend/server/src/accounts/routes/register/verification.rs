use actix_web::{HttpRequest, HttpResponse, Responder, Result};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};

use crate::{
    accounts::{
        datatypes::users::User,
        queries::{
            postgres::{
                create_new_user_in_pg_users_table,
                get_user_from_email_in_pg_users_table,
            },
            redis::get_email_from_token_struct_in_redis,
        },
        emails::compose_register_email_message,
        schema::{
            register::{
                DualVerificationToken, RegisterDetailsRequest,
                RegisterDetailsResponseSchema, RegisterEmailRequestSchema, RegisterEmailResponseSchema,
                VerificationRequest, VerificationRequestSchema, VerificationResponseSchema,
            },
            errors::AccountError,
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
            set_key_value_in_redis,
        },
        email_handler::send_email,
        tokens::generate_opaque_token_of_length,
        validations::{
            validate_email, validate_first_name, validate_last_name, validate_password,
            validate_username,
        },
    },
};


pub async fn post_verify(req_body: Json<VerificationRequest>, req: HttpRequest) -> Result<impl Responder> {
    let VerificationRequest { verification_token } = req_body.into_inner();
    let register_email_token: String = req
        .headers()
        .get("register_email_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    register_verification_functionality(register_email_token, verification_token).await
}

pub async fn link_verify(path: actix_web::web::Path<VerificationRequestSchema>) -> Result<impl Responder> {
    let VerificationRequestSchema {
        header_token,
        verification_token,
    } = path.into_inner();

    register_verification_functionality(header_token, verification_token).await
}

async fn register_verification_functionality(
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
    println!("schema: {:#?}", token_struct_json);

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(con, &token_struct_json) {
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

    // create a new token
    let register_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        con,
        &register_verification_token,
        &email,
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
    res_body.register_response_token = Some(register_verification_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}


