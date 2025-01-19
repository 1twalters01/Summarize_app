pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password is too short".to_string());
    }

    return Ok(());
}

pub fn validate_totp(
    digit1: u32,
    digit2: u32,
    digit3: u32,
    digit4: u32,
    digit5: u32,
    digit6: u32,
) -> Result<(), String> {
    if digit1 > 9 || digit2 > 9 || digit3 > 9 || digit4 > 9 || digit5 > 9 || digit6 > 9 {
        return Err("Invalid digits".to_string());
    }
    return Ok(());
}

pub fn validate_refresh_token(refresh_token: &str) -> Result<(), String> {
    if refresh_token.len() == 32 || refresh_token.chars().all(char::is_alphanumeric) {
        return Err("invalid refresh token".to_string());
    }

    return Ok(());
}