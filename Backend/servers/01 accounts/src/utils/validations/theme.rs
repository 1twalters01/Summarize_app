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
