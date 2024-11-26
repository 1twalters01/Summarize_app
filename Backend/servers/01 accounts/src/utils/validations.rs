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

pub fn validate_refresh_token(refresh_token: &str) -> Result<(), String> {
    if refresh_token.len() < 15 {
        return Err("invalid refresh token".to_string());
    }

    return Ok(());
}

pub fn validate_username(username: &str) -> Result<(), String> {
    if username.len() >= 30 {
        return Err("Username is too long".to_string());
    }

    return Ok(());
}

// pub fn validate_first_name(first_name: String) -> Result<(), String> {
//     if first_name.len() >= 30 {
//         return Err("First name is too long".to_string());
//     }
//
//     if first_name
//         .as_bytes()
//         .iter()
//         .map(|b| b.is_ascii_alphabetic())
//         .collect::<Vec<bool>>()
//         .contains(&false)
//     {
//         return Err("First name is invalid".to_string());
//     }
//
//     return Ok(());
// }
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

use crate::generated::protos::settings::profile::language::request::Language;
pub fn validate_language(language: i32) -> Result<(), String> {
    match Language::try_from(language) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error.to_string()),
    }
}

use crate::generated::protos::settings::profile::theme::request::{
    request::RequestField, Colour, Colours, Custom, Presets,
};
pub fn validate_theme(theme: Option<RequestField>) -> Result<(), String> {
    if theme.is_none() {
        return Err("Invalid theme".to_string());
    }

    match theme.unwrap() {
        RequestField::Custom(Custom { primary, secondary }) => {
            if primary.is_none() || secondary.is_none() {
                return Err("Invalid theme".to_string());
            }

            if let Some(Colours {
                colour_1,
                colour_2,
                colour_3,
                colour_4,
                colour_5,
                colour_6,
            }) = primary
            {
                if check_colour(&colour_1).is_err() {
                    return check_colour(&colour_1);
                }
                if check_colour(&colour_2).is_err() {
                    return check_colour(&colour_2);
                }
                if check_colour(&colour_3).is_err() {
                    return check_colour(&colour_3);
                }
                if check_colour(&colour_4).is_err() {
                    return check_colour(&colour_4);
                }
                if check_colour(&colour_5).is_err() {
                    return check_colour(&colour_5);
                }
                if check_colour(&colour_6).is_err() {
                    return check_colour(&colour_6);
                }
            }

            if let Some(Colours {
                colour_1,
                colour_2,
                colour_3,
                colour_4,
                colour_5,
                colour_6,
            }) = secondary
            {
                if check_colour(&colour_1).is_err() {
                    return check_colour(&colour_1);
                }
                if check_colour(&colour_2).is_err() {
                    return check_colour(&colour_2);
                }
                if check_colour(&colour_3).is_err() {
                    return check_colour(&colour_3);
                }
                if check_colour(&colour_4).is_err() {
                    return check_colour(&colour_4);
                }
                if check_colour(&colour_5).is_err() {
                    return check_colour(&colour_5);
                }
                if check_colour(&colour_6).is_err() {
                    return check_colour(&colour_6);
                }
            }

            return Ok(());
        }
        RequestField::Presets(theme) => match Presets::try_from(theme) {
            Ok(_) => return Ok(()),
            Err(error) => return Err(error.to_string()),
        },
    }
}

fn check_colour(colour: &Option<Colour>) -> Result<(), String> {
    match colour {
        Some(Colour {
            red,
            green,
            blue,
            alpha,
        }) => {
            if red > &255 || green > &255 || blue > &255 || alpha > &255 {
                return Err("Invalid theme".to_string());
            } else {
                return Ok(());
            }
        }
        None => return Err("Invalid theme".to_string()),
    }
}
