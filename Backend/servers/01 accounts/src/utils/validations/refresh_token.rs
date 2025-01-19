pub fn validate_refresh_token(refresh_token: &str) -> Result<(), String> {
    if refresh_token.len() == 32 && refresh_token.chars().all(char::is_alphanumeric) {
        return Ok(());
    }

    return Err("invalid refresh token".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_refresh_token() {
        let tests = vec![
            ("A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6", true),
            ("12345678901234567890123456789012", true),
            ("abcdefghijklmnopqrstuvwxyzABCDEF", true),
            ("A1B2C3D4", false),
            ("A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P67", false),
            ("A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5!", false),
            ("A1B2C3D4E5F6 G7H8I9J0K1L2M3N4O5P6", false),
            ("", false),
        ];

        for (token, expected) in tests {
            assert_eq!(
                validate_refresh_token(token),
                if expected { Ok(()) } else { Err("".to_string()) },
                "Token `{}` was not classified correctly",
                token
            );
        }
    }
}