use crate::user::User;

struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn validate_credentials(&self) -> bool {
        // get user from self.username
        let user: Option<User> = self.get_user();

        match user {
            None => return false,
            Some(user) => {
                // get password from user.password
                // check password
                match user.check_password(self.password.clone()){
                    Ok(_) => return true,
                    Err(_) => return false,
                };
            }
        }

    }

    fn get_user(&self) -> Option<User> {
        todo!()
        // connect to postgres server
        // check if username is a user
    }
}
