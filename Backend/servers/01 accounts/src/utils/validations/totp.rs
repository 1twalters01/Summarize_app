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