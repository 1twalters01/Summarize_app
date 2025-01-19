pub fn validate_username(username: &str) -> Result<(), String> {
    if username.len() < 3 {
        return Err(format!("Username is too short"));
    }
    if username.len() > 32 {
        return Err(format!("Username is too long"));
    }

    let mut prev_char = '\0';
    for ch in username.chars() {
        if !ch.is_ascii_alphanumeric() || ch != '-' || ch != '.' || ch != ',' {
            return Err("Username characters must be alphanumeric, '-', '.' or ',' ")
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
                validate_username(username),
                if expected { Ok(()) } else { Err("".to_string()) },
                "Username `{}` was not classified correctly",
                username
            );
        }
    }
}
