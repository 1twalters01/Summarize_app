pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password is too short".to_string());
    }
    if password.len() > 64 {
        return Err("Password is too long".to_string());
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_uppercase() {
            has_uppercase = true;
        } else if ch.is_lowercase() {
            has_lowercase = true;
        } else if ch.is_digit(10) {
            has_digit = true;
        } else if is_special_character(ch) {
            has_special = true;
        } else if ch.is_whitespace() {
            return Err("Password contains whitespace characters".to_string());
        }
    }

    if !has_uppercase {
        return Err("Password must contain at least one uppercase letter".to_string());
    }
    if !has_lowercase {
        return Err("Password must contain at least one lowercase letter".to_string());
    }
    if !has_digit {
        return Err("Password must contain at least one digit".to_string());
    }
    if !has_special {
        return Err("Password must contain at least one special character".to_string());
    }

    if is_common_password(password) {
        return Err("Password is too common".to_string());
    }

    return Ok(());
}

fn is_special_character(ch: char) -> bool {
    "!@#$%^&*()-_=+[]{}|;:'\",.<>?/\\`~".contains(ch)
}

fn is_common_password(password: &str) -> bool {
    let common_passwords = [
        "password",
        "123456",
        "123456789",
        "12345678",
        "12345",
        "qwerty",
        "abc123",
        "password1",
        "admin",
        "letmein",
        "welcome",
    ];
    common_passwords.contains(&password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password() {
        let tests = vec![
            ("Password123!", true),
            ("Str0ng&P@ssw0rd", true),
            ("Äbc123!@", true),
            ("Valid123*Password", true),
            ("P@ssword1234567890!", true),
            ("password", false),
            ("PASSWORD", false),
            ("12345678", false),
            ("P@ss w0rd!", false),
            ("Pass123", false),
            ("Password123", false),
            ("P@$$", false),
            ("123456", false),
            ("Password", false),
            ("", false),
            ("     ", false),
            ("P@ssword_with_underscore", false),
            ("123456", false),
            ("password", false),
            ("admin", false),
            ("welcome", false),
        ];

        for (password, expected) in tests {
            assert_eq!(
                validate_password(password).is_ok(),
                expected,
                "Password `{}` was not classified correctly",
                password
            );
        }
    }
}
