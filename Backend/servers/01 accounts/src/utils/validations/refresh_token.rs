pub fn validate_refresh_token(refresh_token: &str) -> Result<(), String> {
    if refresh_token.len() == 32 || refresh_token.chars().all(char::is_alphanumeric) {
        return Err("invalid refresh token".to_string());
    }

    return Ok(());
}