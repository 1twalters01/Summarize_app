use serde::{Deserialize, Serialize};

// Settings Structs

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeUsernameRequestStruct {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeUsernameResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ChangeUsernameResponseStruct {
    pub fn new() -> ChangeUsernameResponseStruct {
        ChangeUsernameResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeNameRequestStruct {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeNameResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ChangeNameResponseStruct {
    pub fn new() -> ChangeNameResponseStruct {
        ChangeNameResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeEmailRequestStruct {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeEmailResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ChangeEmailResponseStruct {
    pub fn new() -> ChangeEmailResponseStruct {
        ChangeEmailResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequestStruct {
    pub password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ChangePasswordResponseStruct {
    pub fn new() -> ChangePasswordResponseStruct {
        ChangePasswordResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAccountRequestStruct {
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleTotpRequestStruct {
    pub totp: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleTotpResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ToggleTotpResponseStruct {
    pub fn new() -> ToggleTotpResponseStruct {
        ToggleTotpResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetThemeResponseStruct {
    pub settings_error: SettingsError,
    pub theme: Theme,
}

impl GetThemeResponseStruct {
    pub fn new() -> GetThemeResponseStruct {
        GetThemeResponseStruct {
            settings_error: SettingsError::new(),
            theme: Theme::Dark,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeThemeRequestStruct {
    pub theme: Theme,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Custom(CustomTheme),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTheme {
    pub primary_colours: PrimaryColours,
    pub secondary_colours: SecondaryColours,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimaryColours {
    pub primary_colour_1: Colour,
    pub primary_colour_2: Colour,
    pub primary_colour_3: Colour,
    pub primary_colour_4: Colour,
    pub primary_colour_5: Colour,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondaryColours {
    pub secondary_colour_1: Colour,
    pub secondary_colour_2: Colour,
    pub secondary_colour_3: Colour,
    pub secondary_colour_4: Colour,
    pub secondary_colour_5: Colour,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Colour {
    pub red: i8,
    pub green: i8,
    pub blue: i8,
    pub alpha: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeThemeResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ChangeThemeResponseStruct {
    pub fn new() -> ChangeThemeResponseStruct {
        ChangeThemeResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeLanguageRequestStruct {
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeLanguageResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

impl ChangeLanguageResponseStruct {
    pub fn new() -> ChangeLanguageResponseStruct {
        ChangeLanguageResponseStruct {
            settings_error: SettingsError::new(),
            success: false,
        }
    }
}
