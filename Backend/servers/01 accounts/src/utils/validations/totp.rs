pub fn validate_totp(digits: &[u32]) -> Result<(), String> {
    // Ensure all digits are between 0 and 9
    if digits.len() != 6 {
        return Err("Totp must have 6 digits")
    }
    if digits.iter().any(|&digit| digit > 9) {
        return Err("Digits must be between 0 and 9.".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_totp() {
        let tests = vec![
            (vec![1, 2, 3, 4, 5, 6], true),
            (vec![0, 0, 0, 0, 0, 0], true),
            (vec![9, 9, 9, 9, 9, 9], true),
            (vec![1, 2, 3, 4, 5], false),
            (vec![1, 2, 3, 4, 5, 6, 7], false),
            (vec![1, 2, 3, 4, 5, 10], false),
            (vec![], false),
        ];

        for (digits, expected) in tests {
            assert_eq!(
                validate_totp(&digits).is_ok(),
                expected,
                "TOTP `{:?}` was not classified correctly",
                digits
            );
        }
    }
}
