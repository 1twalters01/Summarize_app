use actix_web::{
    http::StatusCode,
    HttpResponse,
};
use crate::{
    generated::protos::accounts::{
        login::{
            email::response as login_email_response,
            password::response as login_password_response,
            totp::response as login_totp_response,
            // refresh::response as login_refresh_response,
        },
        register::{
            email::response as register_email_response,
            verification::response as register_verification_response,
            details::response as register_details_response,
        },
        password_reset::{
            email::response as password_reset_email_response,
            verification::response as password_reset_verification_response,
            password::response as password_reset_password_response,
        }
    }
};

pub struct ResponseService;

impl ResponseService {
    pub fn create_error_response(
        error: AppError,
        status: StatusCode,
    ) -> HttpResponse {
        let response = match error {
            AppError::LoginEmail(err) => {
                login_email_response::Response {
                    response_field: Some(login_email_response::ResponseField::Error(err as i32)),
                };
            }
            AppError::LoginPassword(err) => {
                login_password_response::Response {
                    response_field: Some(login_password_response::ResponseField::Error(err as i32)),
                };
            }
            AppError::RegisterEmail(err) => {
                register_email_response::Response {
                    response_field: Some(register_email_response::ResponseField::Error(err as i32)),
                };
            }
        };

        httpresponse::build(status)
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response)
    }

    pub fn create_success_response(
        response: AppResponse,
        status: StatusCode,
    ) -> HttpResponse {
        let response = match response {
            AppResponse::LoginEmail(res) => res,
            AppResponse::LoginPassword(res) => res,
            AppResponse::RegisterEmail(res) => res,
        };

        httpresponse::build(status)
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response)
    }
}