pub fn compose_register_email_message(verify_token: &str, register_email_token: &str) -> String {
    let message: String = format!("Token: {}", token);
    return message
}

pub fn send_email(message: &str, email: &str) -> Result<(), String> {
    return Ok(())
}
