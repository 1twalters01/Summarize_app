use idna;

pub fn validate_username(username: &str) -> Result<(), String> {
    if username.len() < 3 {
        return Err(format!("Username is too short"));
    }
    if username.len() > 32 {
        return Err(format!("Username is too long"));
    }

    let ascii_username = match idna::domain_to_ascii(username) {
        Ok(username) => username,
        Err(_) => return Err(format!("Invalid character(s) in username")), // Invalid Unicode domain
    };

    for ch in ascii_username.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '-' && ch != '_' && ch != '.' {
            return Err("Username characters must be alphanumeric, '-', '.' or ',' ".to_string());
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        let tests = vec![
            // Valid usernames
            ("user123", true),
            ("user_name", true),
            ("user-name", true),
            ("user.name", true),
            ("u123", true),
            ("abc.def-ghi_jkl", true),
            ("Üser123", true),
            ("मेरा123", true),
            ("a.b_c-1", true),
            // Invalid usernames
            ("", false),
            ("ab", false),
            ("user@name", false),
            ("user name", false),
            ("this_username_is_way_too_long_to_be_valid", false), // Too long
        ];

        for (username, expected) in tests {
            assert_eq!(
                validate_username(username).is_ok(),
                expected,
                "Username `{}` was not classified correctly",
                username
            );
        }
    }
}
