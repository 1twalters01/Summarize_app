pub fn validate_email(email: String) -> Result<(), String> {
    if email.contains("@") | email.contains(".") != true {
        return Err("Invalid email".to_string());
    }

    if email.len() < 6 {
        return Err("Email is too short".to_string());
    }
    
    return Ok(());
}

pub fn validate_password(password: String) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password is too short".to_string());
    }
    
    return Ok(());
}

pub fn validate_totp(totp: String) -> Result<(), String> {
    if totp.len() != 6 {
        return Err("Totp incorrect".to_string());
    }

    if totp.as_bytes().iter().map(|b|b.is_ascii_digit()).collect::<Vec<bool>>().contains(&false) {
        return Err("Totp incorrect".to_string());
    }
    
    return Ok(());
}
