use crate::generated::protos::{
    accounts::{
        login::{
            email::response as login_email_response, password::response as login_password_response,
            refresh::response as login_refresh_response, totp::response as login_totp_response,
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
        captcha::{
            get::response as captcha_get_response,
            verification::response as captcha_verification_response,
        }
    },
    settings::profile::{
        confirmation as confirmation_response, email::response as change_email_response,
        language::response as change_language_response, name::response as change_name_response,
        password::response as change_password_response, theme::response as change_theme_response,
        totp::response as change_totp_response, username::response as change_username_response,
    },
};

pub enum AppError {
    // Login
    LoginEmail(login_email_response::Error),
    LoginPassword(login_password_response::Error),
    LoginTotp(login_totp_response::Error),
    LoginRefresh(login_refresh_response::Error),

    // Register
    RegisterEmail(register_email_response::Error),
    RegisterVerification(register_verification_response::Error),
    RegisterDetails(register_details_response::Error),

    // Password Reset
    PasswordResetEmail(password_reset_email_response::Error),
    PasswordResetVerification(password_reset_verification_response::Error),
    PasswordResetPassword(password_reset_password_response::Error),

    // Captch
    CaptchaGet(captcha_get_response::Error),
    CaptchaVerification(captcha_verification_response::Error)

    // Settings
    Confirmation(confirmation_response::Error),
    ChangeEmail(change_email_response::Error),
    ChangeLanguage(change_language_response::Error),
    ChangeName(change_name_response::Error),
    ChangePassword(change_password_response::Error),
    ChangeTheme(change_theme_response::Error),
    ChangeTotp(change_totp_response::Error),
    ChangeUsername(change_username_response::Error),
}

pub enum AppResponse {
    // Login
    LoginEmail(login_email_response::Response),
    LoginPassword(login_password_response::Response),
    LoginTotp(login_totp_response::Response),
    LoginRefresh(login_refresh_response::Response),

    // Register
    RegisterEmail(register_email_response::Response),
    RegisterVerification(register_verification_response::Response),
    RegisterDetails(register_details_response::Response),

    // Password Reset
    PasswordResetEmail(password_reset_email_response::Response),
    PasswordResetVerification(password_reset_verification_response::Response),
    PasswordResetPassword(password_reset_password_response::Response),

    // Captch
    CaptchaGet(captcha_get_response::Response),
    CaptchaVerification(captcha_verification_response::Response,)

    // Settings
    Confirmation(confirmation_response::Response),
    ChangeEmail(change_email_response::Response),
    ChangeLanguage(change_language_response::Response),
    ChangeName(change_name_response::Response),
    ChangePassword(change_password_response::Response),
    ChangeTheme(change_theme_response::Response),
    ChangeTotp(change_totp_response::Response),
    ChangeUsername(change_username_response::Response),
}
