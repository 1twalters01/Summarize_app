use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    result::Result,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Password {
    password_hash: String,
}

impl Password {
    pub fn new(password: String) -> Result<Self, Error> {
        match validate_password(&password) {
            Ok(_) => return Ok(hash_password(password)),
            Err(err) => return Err(err),
        }
    }

    pub fn get_password_string(&self) -> String {
        return self.password_hash.clone();
    }

    pub fn set_password(&mut self, password: String) -> Result<(), Error> {
        match validate_password(&password) {
            Ok(_) => {
                *self = hash_password(password);

                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }

    pub fn check_password(&self, password: &str) -> Result<(), argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(&self.password_hash).unwrap();
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
    }
}

fn validate_password(password: &str) -> Result<(), Error> {
    if password.len() < 8 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Password must be at least 8 characters long",
        ));
    }

    return Ok(());
}

fn hash_password(password: String) -> Password {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    return Password { password_hash };
}
