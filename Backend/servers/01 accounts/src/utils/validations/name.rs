pub fn validate_name(name: &str) -> Result<(), String> {
    if name.len() < 1 {
        return Err("Name must contain at least 1 character".to_string());
    }
    if name.len() > 100 {
        return Err("Name must not exceed 100 characters".to_string());
    }

    if local.starts_with('-') || local.ends_with('-') {
        return false;
    }
    if local.starts_with('\'') || local.ends_with('\'') {
        return false;
    }

    let mut prev_char = '\0';
    for ch in name.chars() {
        if !ch.is_alphabetic() || ch == '-' || is_special_character(ch) {
            return Err("Name contains invalid characters")
        }
        if is_special_character(ch) && is_special_character(prev_char) {
            return Err("Name cannot contain consecutive special characters")
        }
    }

    return Ok(());
}

pub fn is_special_character(ch: char) {
    ch == '-' || ch == '\''
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        let tests = vec![
            // Valid names
            ("John", true),
            ("John Doe", true),
            ("O'Connor", true),
            ("Jean-Luc", true),
            ("Renée", true),
            ("李小龍", true),
            ("A", true),
            ("Léo", true),
            ("Mary Jane Watson", true),

            // Invalid names
            ("", false),
            (" ", false),
            ("John--Doe", false),
            ("O''Connor", false),
            ("John  Doe", false),
            ("-John", false),
            ("John-", false),
            ("'John", false),
            ("John'", false),
            (" John", false),
            ("John ", false),
            ("John@Doe", false),
            ("John123", false),
            ("John!", false),
            ("ThisNameIsWayTooLongToBeValidBecauseItExceedsTheMaximumLengthOfOneHundredCharacters", false),
        ];

        for (name, expected) in tests {
            assert_eq!(
                validate_name(name),
                if expected { Ok(()) } else { Err("".to_string()) },
                "Name `{}` was not classified correctly",
                name
            );
        }
    }
}
