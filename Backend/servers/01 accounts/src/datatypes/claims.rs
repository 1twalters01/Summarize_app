use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaptchaClaims {
    pub ip: SocketAddr,
    pub exp: usize,
}
