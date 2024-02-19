use std::{
    time::{Instant, SystemTime},
    io::{Error, ErrorKind},
};

use uuid::Uuid;

use crate::password::Password;

#[derive(Debug)]
pub struct User {
    id: Uuid,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: Password, // TODO
    totp: Option<String>,
    created_at: SystemTime,
    last_login: SystemTime, // TODO
    groups: Vec<String>, // TODO
    user_permissions: Vec<String>, // TODO
    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
    is_authenticated: bool,
    is_anonymous: bool,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Result<Self, Error> {
        match Password::new(password) {
            Ok(password) => {
                let password = Self {
                    id: Uuid::new_v4(),
                    username,
                    first_name: String::new(),
                    last_name: String::new(),
                    email,
                    password,
                    totp: None, 
                    created_at: SystemTime::now(),
                    last_login: SystemTime::now(),
                    groups: Vec::new(), //todo!(),
                    user_permissions: Vec::new(), //todo!(),
                    is_active: true,
                    is_staff: false,
                    is_superuser: false,
                    is_authenticated: false,
                    is_anonymous: false,
                };

                return Ok(password);

            },
            Err(err) => return Err(err),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.to_owned()
    }

    pub fn get_username(&self) -> String {
        return self.username.to_owned();
    }

    pub fn get_email(&self) -> String {
        return self.email.to_owned();
    }

    pub fn get_full_name(&self) -> String {
        return format!("{} {}", self.first_name.to_owned(), self.last_name.to_owned());
    }

    pub fn get_first_name(&self) -> String {
        return self.first_name.to_owned();
    }

    fn get_last_name(&self) -> String {
        return self.last_name.to_owned();
    }

    fn set_password(&mut self, password: String) -> Result<(), Error> {
        return Password::set_password(&mut self.password, password);
    }

    fn set_totp(&mut self, totp: String) -> bool {
        self.totp = Some(totp.clone());
        if self.totp == Some(totp) {
            return true;
        }
        return false;
    }

    pub fn check_totp(&mut self, totp: i64) -> bool {
        if self.generate_totp_i64() == totp {
            return true;
        }
        return false;
    }

    pub fn totp_required(&mut self) -> bool {
        if self.totp == None {
            return true
        } else { return false; };
    }

    fn generate_totp_i64(&mut self) -> i64 {
        todo!();
    }

    pub fn check_password(&self, password: String) -> Result<(), Error> {
        match Password::check_password(&self.password, password) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid password: {}", err)))
   

        }
    }
    
    fn get_user_permissions(&self) {

    }
    
    fn get_groups(&self) {

    }
    
    fn has_permission(&self, permission: String) -> bool {
        if self.user_permissions.contains(&permission) {
            return true;
        }
        return false;
    }
    
    fn has_permissions(&self, permissions: Vec<String>) -> Vec<bool> {
        return permissions.into_iter().map(|permission| self.user_permissions.contains(&permission)).collect();
    }
}
