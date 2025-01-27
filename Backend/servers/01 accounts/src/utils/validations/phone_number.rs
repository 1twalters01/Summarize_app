pub fn validate_phone_number(phone_number: &str) -> Result<(), String> {
    let phone_number = phone_number.trim();

    let valid_chars = "+0123456789()- ";
    let mut digit_count = 0;
    let mut open_parentheses = 0;
    let mut last_char = '\0';

    // Check each character
    for ch in phone.chars() {
        if !valid_chars.contains(ch) {
            return Err(format!("Invalid character '{}' in phone number", ch));
        }

        if ch.is_ascii_digit() {
            digit_count += 1;
        }

        if ch == '(' {
            open_parentheses += 1;
            // '(' should not appear after a digit or at the end
            if last_char.is_ascii_digit() {
                return Err("Invalid '(' after a digit".to_string());
            }
        } else if ch == ')' {
            open_parentheses -= 1;
            if open_parentheses < 0 {
                return Err("Unmatched ')' in phone number".to_string());
            }
        }

        last_char = ch;
    }

    if open_parentheses != 0 {
        return Err("Unbalanced parentheses in phone number".to_string());
    }

    if digit_count < 7 || digit_count > 15 {
        return Err(format!(
            "Invalid digit count: {}. Must be between 7 and 15 digits",
            digit_count
        ));
    }

    if phone.starts_with('+') && !phone[1..].chars().next().unwrap().is_ascii_digit() {
        return Err("Invalid '+' usage in phone number".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_phone_number() {
        let tests =
            vec![
            // Valid phone numbers
            ("+1234567890", true),
            ("+1 123 456 7890", true),
            ("+44 (20) 7946 0958", true),
            ("123-456-7890", true),
            ("(123) 456-7890", true),
            ("+1 (123) 456-7890", true),

            // Invalid phone numbers
            ("123", false),
            ("+1234567890123456", false),
            ("abcd1234", false),
            ("+1-800-FLOWERS", false),
            ("+1 (123) 456 7890)", false),
            ("123) 456 7890", false),
            ("(1234567890", false),
            ("+-1234567890", false),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                validate_phone_number(input).is_ok(),
                expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }
}