pub fn validate_name(name: &str) -> Result<(), String> {
    if name.len() >= 30 {
        return Err("Name is too long".to_string());
    }

    if name
        .as_bytes()
        .iter()
        .map(|b| b.is_ascii_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&false)
    {
        return Err("Name is invalid".to_string());
    }

    return Ok(());
}

pub fn validate_email(email: &str) -> Result<(), String> {
    if email.contains("@") | email.contains(".") != true {
        return Err("Invalid email".to_string());
    }

    if email.len() < 6 {
        return Err("Email is too short".to_string());
    }

    return Ok(());
}

pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password is too short".to_string());
    }

    return Ok(());
}

pub fn validate_totp(
    digit1: u32,
    digit2: u32,
    digit3: u32,
    digit4: u32,
    digit5: u32,
    digit6: u32,
) -> Result<(), String> {
    if digit1 > 9 || digit2 > 9 || digit3 > 9 || digit4 > 9 || digit5 > 9 || digit6 > 9 {
        return Err("Invalid digits".to_string());
    }
    return Ok(());
}

pub fn validate_username(username: &str) -> Result<(), String> {
    if username.len() >= 30 {
        return Err("Username is too long".to_string());
    }

    return Ok(());
}

pub fn validate_first_name(first_name: String) -> Result<(), String> {
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

pub fn validate_last_name(last_name: String) -> Result<(), String> {
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
