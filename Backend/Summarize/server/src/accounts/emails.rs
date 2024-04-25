use crate::accounts::datatypes::users::User;

pub fn compose_register_email_message(verify_token: &str, register_email_token: &str) -> String {
    let message: String = format!("Verify Token: {}, Register Email Token: {}", verify_token, register_email_token);
    return message
}

pub fn compose_password_reset_email_message(password_reset_response_token: &str, user: &User) -> String {
    let message: String = format!("Token: {}", password_reset_response_token);
    return message
}

pub fn send_email(message: &str, email: &str) -> Result<(), String> {
    return Ok(())
}
