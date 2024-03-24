use serde::{Serialize, Deserialize};

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
    pub totp: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Logout {

}

#[derive(Serialize, Deserialize)]
pub struct Register {

}
