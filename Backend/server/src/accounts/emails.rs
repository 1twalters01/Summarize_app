use crate::accounts::datatypes::users::User;
use crate::utils::email_handler::EmailMessage;


pub fn compose_register_email_message(
    verify_token: &str,
    register_email_token: &str,
) -> EmailMessage {
    let message: EmailMessage = EmailMessage {
        subject: String::from("Register to Summarize"),
        body: format!(
            "<h1>Summarize</h1><p>Verify Token: {}</p><p>Register Email Token: {}</p>",
            verify_token, register_email_token
        ),
    };
    return message;
}

pub fn compose_password_reset_email_message(
    password_reset_response_token: &str,
    user: &User,
) -> EmailMessage {
    let message = EmailMessage {
        subject: String::from("Summarize Password Reset"),
        body: format!("Token: {}", password_reset_response_token),
    };
    return message;
}
