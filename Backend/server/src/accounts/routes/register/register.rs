use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};

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


#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use dotenv::dotenv;
    use serde_json::json;

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_without_username_password_confirmation_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_token_with_header_token_username_password_without_confirmation_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_without_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_first_without_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_without_header_token_username_password_confirmation_with_first_last() {
    }


    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_without_username_password_confirmation_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_token_with_header_token_username_password_without_confirmation_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_without_first_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_first_without_last() {
    }
    
    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_without_header_token_username_password_confirmation_with_first_last() {
    }
}
