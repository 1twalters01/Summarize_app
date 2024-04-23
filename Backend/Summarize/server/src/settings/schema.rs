use serde::{Serialize, Deserialize};

// Settings Structs

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SettingsError {
    pub is_error: bool,
    pub error_message: Option<String>,
}

impl SettingsError {
    pub fn new() -> SettingsError {
        SettingsError {
            is_error: false,
            error_message: None,
        }
    }
}











#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeUsernameRequestStruct {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeUsernameResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

impl ChangeUsernameResponseStruct {
    pub fn new() -> ChangeUsernameResponseStruct {
        ChangeUsernameResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}



#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeEmailRequestStruct {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeEmailResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

impl ChangeEmailResponseStruct {
    pub fn new() -> ChangeEmailResponseStruct {
        ChangeEmailResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}



#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequestStruct {
    pub password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangePasswordResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

impl ChangePasswordResponseStruct {
    pub fn new() -> ChangePasswordResponseStruct {
        ChangePasswordResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct DeleteAccountRequestStruct {
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct DeleteAccountResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl DeleteAccountResponseStruct {
    pub fn new() -> DeleteAccountResponseStruct {
        DeleteAccountResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}





#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ToggleTotpRequestStruct {
    pub totp: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ToggleTotpResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

impl ToggleTotpResponseStruct {
    pub fn new() -> ToggleTotpResponseStruct {
        ToggleTotpResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}





#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeThemeRequestStruct {
    pub theme: Theme,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Custom(CustomTheme),
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct CustomTheme {
    pub primary_colours: PrimaryColours,
    pub secondary_colours: SecondaryColours,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PrimaryColours {
    primary_colour_1: Colour,
    primary_colour_2: Colour,
    primary_colour_3: Colour,
    primary_colour_4: Colour,
    primary_colour_5: Colour,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SecondaryColours {
    secondary_colour_1: Colour,
    secondary_colour_2: Colour,
    secondary_colour_3: Colour,
    secondary_colour_4: Colour,
    secondary_colour_5: Colour,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Colour {
    red: i8,
    green: i8,
    blue: i8,
    alpha: i8,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeThemeResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

impl ChangeThemeResponseStruct {
    pub fn new() -> ChangeThemeResponseStruct {
        ChangeThemeResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

