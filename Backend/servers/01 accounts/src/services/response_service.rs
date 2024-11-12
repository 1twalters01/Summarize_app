use actix_protobuf::ProtoBufResponseBuilder;
use actix_web::{http::StatusCode, HttpResponse, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::{
        login::{
            email::response as login_email_response,
            password::response as login_password_response,
            totp::response as login_totp_response,
            refresh::response as login_refresh_response,
        },
        password_reset::{
            email::response as password_reset_email_response,
            password::response as password_reset_password_response,
            verification::response as password_reset_verification_response,
        },
        register::{
            details::response as register_details_response,
            email::response as register_email_response,
            verification::response as register_verification_response,
        },
    },
};

pub struct ResponseService;

impl ResponseService {
    pub fn create_error_response(error: AppError, status: StatusCode) -> Result<HttpResponse> {
        // ) -> impl Responder {
        let response = match error {
            AppError::LoginEmail(err) => {
                login_email_response::Response {
                    response_field: Some(login_email_response::response::ResponseField::Error(
                        err as i32,
                    )),
                };
            }
            AppError::LoginPassword(err) => {
                login_password_response::Response {
                    response_field: Some(login_password_response::response::ResponseField::Error(
                        err as i32,
                    )),
                };
            }
            AppError::LoginTotp(err) => {
                login_totp_response::Response {
                    response_field: Some(login_totp_response::response::ResponseField::Error(
                        err as i32,
                    )),
                };
            }
            AppError::LoginRefresh(err) => {
                login_refresh_response::Response {
                    response_field: Some(login_refresh_response::response::ResponseField::Error(err as i32)),
                };
            },
            AppError::RegisterEmail(err) => {
                register_email_response::Response {
                    response_field: Some(register_email_response::response::ResponseField::Error(
                        err as i32,
                    )),
                };
            }
            AppError::RegisterVerification(err) => {
                register_verification_response::Response {
                    response_field: Some(
                        register_verification_response::response::ResponseField::Error(err as i32),
                    ),
                };
            }
            AppError::RegisterDetails(err) => {
                register_details_response::Response {
                    response_field: Some(
                        register_details_response::response::ResponseField::Error(err as i32),
                    ),
                };
            }
            AppError::PasswordResetEmail(err) => {
                password_reset_email_response::Response {
                    response_field: Some(
                        password_reset_email_response::response::ResponseField::Error(err as i32),
                    ),
                };
            }
            AppError::PasswordResetVerification(err) => {
                password_reset_verification_response::Response {
                    response_field: Some(
                        password_reset_verification_response::response::ResponseField::Error(
                            err as i32,
                        ),
                    ),
                };
            }
            AppError::PasswordResetPassword(err) => {
                password_reset_password_response::Response {
                    response_field: Some(
                        password_reset_password_response::response::ResponseField::Error(
                            err as i32,
                        ),
                    ),
                };
            }
        };

        HttpResponse::build(status)
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response)
    }

    pub fn create_success_response(
        response: AppResponse,
        status: StatusCode,
    ) -> Result<HttpResponse> {
        // ) -> impl Responder {
        match response {
            AppResponse::LoginEmail(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::LoginPassword(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::LoginTotp(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::LoginRefresh(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::RegisterEmail(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::RegisterVerification(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::RegisterDetails(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::PasswordResetEmail(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::PasswordResetVerification(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
            AppResponse::PasswordResetPassword(res) => HttpResponse::build(status)
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(res),
        }
    }
}
