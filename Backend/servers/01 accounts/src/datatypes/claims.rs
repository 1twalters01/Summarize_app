use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Claims for authentication jwt
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserClaims {
    pub sub: String,
    pub exp: usize,
}

/// Claims for captcha jwt
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaptchaClaims {
    pub ip: SocketAddr,
    pub exp: usize,
}
