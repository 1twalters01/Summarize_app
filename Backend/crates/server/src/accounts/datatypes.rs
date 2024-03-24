use serde::{Serialize, Deserialize};

// Login Structs
#[derive(Debug)]
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginEmail {
    pub email: String,
}

#[derive(Debug)]
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginPassword {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Clone, Serialize, Deserialize)]
pub struct LoginTotp {
    pub email: String,
    pub password: String,
    pub totp: String,
}


// Register Structs
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RegisterEmail {
    pub email: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RegisterVerify {
    pub email: String,
    pub token: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RegisterDetails {
    pub email: String,
    pub token: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Activate {

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UsernameReset {

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UsernameResetConfirm {

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PasswordReset {

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PasswordResetConfirm {

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Logout {

}


