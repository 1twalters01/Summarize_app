use crate::generated::protos::accounts::{
    login::{
        email::response as login_email_response,
        password::response as login_password_response,
        totp::response as login_totp_response,
        // refresh::response as login_refresh_response,
    },
    password_reset::{
        email::response as password_reset_email_response,
        password::response as password_reset_password_response,
        verification::response as password_reset_verification_response,
    },
    register::{
        details::response as register_details_response, email::response as register_email_response,
        verification::response as register_verification_response,
    },
};

pub enum AppError {
    // Login
    LoginEmail(login_email_response::Error),
    LoginPassword(login_password_response::Error),
    LoginTotp(login_totp_response::Error),
    // LoginRefresh(login_refresh_response::Error),

    // Register
    RegisterEmail(register_email_response::Error),
    RegisterVerification(register_verification_response::Error),
    RegisterDetails(register_details_response::Error),

    // Password Reset
    PasswordResetEmail(password_reset_email_response::Error),
    PasswordResetVerification(password_reset_verification_response::Error),
    PasswordResetPassword(password_reset_password_response::Error),
}

pub enum AppResponse {
    // Login
    LoginEmail(login_email_response::Response),
    LoginPassword(login_password_response::Response),
    LoginTotp(login_totp_response::Response),
    // LoginRefresh(login_refresh_response::Response),

    // Register
    RegisterEmail(register_email_response::Response),
    RegisterVerification(register_verification_response::Response),
    RegisterDetails(register_details_response::Response),

    // Password Reset
    PasswordResetEmail(password_reset_email_response::Response),
    PasswordResetVerification(password_reset_verification_response::Response),
    PasswordResetPassword(password_reset_password_response::Response),
}
