use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    time::SystemTime,
};

use crate::{
    accounts::{
        datatypes::{passwords::Password, totp::Totp},
        queries::postgres::get_user_from_uuid_in_pg_users_table,
    },
    utils::database_connections::create_pg_pool_connection,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    uuid: Uuid,
    email: String,
    password: Password,
    totp: Totp,

    username: String,
    first_name: Option<String>,
    last_name: Option<String>,

    created_at: SystemTime,
    last_login: SystemTime,

    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
    is_authenticated: bool,
    is_anonymous: bool,

    groups: Vec<String>,           // TODO
    user_permissions: Vec<String>, // TODO
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<Self, Error> {
        match Password::from_password(password) {
            Ok(password) => {
                let user = Self {
                    uuid: Uuid::new_v4(),
                    username,
                    first_name,
                    last_name,
                    email,
                    password,
                    totp: Totp::new(),
                    created_at: SystemTime::now(),
                    last_login: SystemTime::now(),
                    groups: Vec::new(),           //todo!(),
                    user_permissions: Vec::new(), //todo!(),
                    is_active: true,
                    is_staff: false,
                    is_superuser: false,
                    is_authenticated: false,
                    is_anonymous: false,
                };

                return Ok(user);
            }
            Err(err) => return Err(err),
        }
    }

    pub fn from_all(
        id: Uuid,
        username: String,
        email: String,
        password_hash: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<Self, Error> {
        let password = match Password::from_hash(password_hash) {
            Ok(password) => password,
            Err(err) => return Err(err),
        };

        return Ok(Self {
            uuid: id,
            email,
            username,
            first_name,
            last_name,
            password,
            totp: Totp::new(),
            created_at: SystemTime::now(),
            last_login: SystemTime::now(),
            groups: Vec::new(),           //todo!(),
            user_permissions: Vec::new(), //todo!(),
            is_active: true,
            is_staff: false,
            is_superuser: false,
            is_authenticated: false,
            is_anonymous: false,
        });
    }

    pub async fn from_uuid_str(uuid: &str) -> Result<Option<Self>, sqlx::Error> {
        let pool = create_pg_pool_connection().await;
        let get_user_result: Result<Option<User>, sqlx::Error> =
            get_user_from_uuid_in_pg_users_table(&pool, uuid).await;

        match get_user_result {
            Ok(user) => return Ok(user),
            Err(err) => return Err(err),
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid.to_owned()
    }

    pub fn get_username(&self) -> String {
        return self.username.to_owned();
    }

    pub fn get_full_name(&self) -> String {
        let first_name = self.get_first_name();
        let last_name = self.get_last_name();

        return format!("{} {}", first_name.to_owned(), last_name.to_owned());
    }

    pub fn get_first_name(&self) -> String {
        let first_name = match self.first_name.clone() {
            Some(first_name) => first_name,
            None => String::new(),
        };

        return first_name;
    }

    pub fn get_last_name(&self) -> String {
        let last_name = match self.last_name.clone() {
            Some(last_name) => last_name,
            None => String::new(),
        };
        return last_name;
    }

    pub fn get_email(&self) -> String {
        return self.email.to_owned();
    }

    pub fn get_password(&self) -> String {
        return self.password.get_password_string();
    }

    pub fn set_password(&mut self, password: String) -> Result<(), Error> {
        return Password::set_password(&mut self.password, password);
    }

    pub fn check_password(&self, password: &str) -> Result<(), Error> {
        match Password::check_password(&self.password, password) {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid password: {}", err),
                ))
            }
        }
    }

    pub fn is_totp_activated(&self) -> bool {
        return self.totp.fields.is_some();
    }

    // fn set_totp(&mut self, url: String) {
    //     self.totp.set_url(url);
    // }

    pub fn check_totp(&mut self, totp: u32) -> bool {
        if self.generate_totp_i64() == totp {
            return true;
        }
        return false;
    }

    pub fn totp_required(&mut self) -> bool {
        if self.totp.fields.is_none() {
            return true;
        }
        return false;
    }

    fn generate_totp_i64(&mut self) -> u32 {
        todo!();
    }

    // fn get_created_time(&self) -> SystemTime {
    //     return self.created_at;
    // }

    // fn get_last_login_time(&self) -> SystemTime {
    //     return self.last_login;
    // }

    // fn get_groups(&self) -> Vec<String> {
    //     return self.groups.clone();
    // }

    // fn get_user_permissions(&self) -> Vec<String> {
    //     return self.user_permissions.clone();
    // }

    // fn has_permission(&self, permission: String) -> bool {
    //     if self.user_permissions.contains(&permission) {
    //         return true;
    //     }
    //     return false;
    // }

    // fn has_permissions(&self, permissions: Vec<String>) -> Vec<bool> {
    //     return permissions
    //         .into_iter()
    //         .map(|permission| self.user_permissions.contains(&permission))
    //         .collect();
    // }

    // fn get_is_user_active(&self) -> bool {
    //     return self.is_active;
    // }

    // fn set_is_user_active(&mut self, is_active: bool) -> bool {
    //     self.is_active = is_active;
    //     if self.is_active == is_active {
    //         return true;
    //     }
    //     return false;
    // }

    // fn get_is_user_staff(&self) -> bool {
    //     return self.is_staff;
    // }

    // fn get_is_user_superuser(&self) -> bool {
    //     return self.is_superuser;
    // }

    // fn get_is_user_authenticated(&self) -> bool {
    //     return self.is_authenticated;
    // }

    // fn set_is_user_authenticated(&mut self, is_authenticated: bool) -> bool {
    //     self.is_authenticated = is_authenticated;
    //     if self.is_authenticated == is_authenticated {
    //         return true;
    //     }
    //     return false;
    // }

    // fn get_is_user_anonymous(&self) -> bool {
    //     return self.is_anonymous;
    // }
}
