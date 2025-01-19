pub fn validate_username(username: &str) -> Result<(), String> {
    if username.len() >= 30 {
        return Err("Username is too long".to_string());
    }

    return Ok(());
}