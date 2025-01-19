pub fn validate_name(first_name: &str, last_name: &str) -> Result<(), String> {
    if first_name.len() >= 30 {
        return Err("First name is too long".to_string());
    }

    if last_name.len() >= 39 {
        return Err("Last name is too long".to_string());
    }

    if first_name
        .as_bytes()
        .iter()
        .map(|b| b.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        return Err("First name is invalid".to_string());
    }

    if last_name
        .as_bytes()
        .iter()
        .map(|b| b.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        return Err("Last name is invalid".to_string());
    }

    return Ok(());
}

pub fn validate_first_name(first_name: &str) -> Result<(), String> {
    if first_name.len() >= 30 {
        return Err("First name is too long".to_string());
    }

    if first_name
        .as_bytes()
        .iter()
        .map(|b| b.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        return Err("First name is invalid".to_string());
    }

    return Ok(());
}

pub fn validate_last_name(last_name: &str) -> Result<(), String> {
    if last_name.len() >= 30 {
        return Err("Last name is too long".to_string());
    }

    if last_name
        .as_bytes()
        .iter()
        .map(|b| b.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        return Err("Last name is invalid".to_string());
    }

    return Ok(());
}