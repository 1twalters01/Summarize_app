pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password is too short".to_string());
    }

    return Ok(());
}



