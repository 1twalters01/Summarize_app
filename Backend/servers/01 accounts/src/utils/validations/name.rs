pub fn validate_name(name: &str) -> Result<(), String> {
    if name.len() < 1 {
        return Err("Name must contain at least 1 character".to_string());
    }
    if name.len() > 50 {
        return Err("Name must not exceed 80 characters".to_string());
    }

    if is_special_character(name.chars().next().unwrap())
        || is_special_character(name.chars().last().unwrap())
    {
        return Err("Name must not start or end with a special character".to_string());
    }
    if name.starts_with('\'') || name.ends_with('\'') {
        return Err("Name must not start or end with a special character".to_string());
    }

    let mut prev_char = '\0';
    for ch in name.chars() {
        if !ch.is_alphabetic() && ch != '-' && !is_special_character(ch) {
            return Err("Name contains invalid characters".to_string());
        }
        if is_special_character(ch) && is_special_character(prev_char) {
            return Err("Name cannot contain consecutive special characters".to_string());
        }
        prev_char = ch;
    }

    return Ok(());
}

pub fn is_special_character(ch: char) -> bool {
    ch == '-' || ch == '\'' || ch == ' '
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
            (
                "ThisNameIsWayTooLongToBeValidBecauseItExceedsTheMaximumLengthOfFiftyCharacters",
                false,
            ),
        ];

        for (name, expected) in tests {
            assert_eq!(
                validate_name(name).is_ok(),
                expected,
                "Name `{}` was not classified correctly",
                name
            );
        }
    }
}
