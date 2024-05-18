use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;
use crate::accounts::datatypes::users::User;


pub fn generate_opaque_token_of_length(length: i64) -> String {
    let mut rng = thread_rng();
    let bytes: Vec<u8> = (0..length)
       .map(|_| rng.sample(Alphanumeric))
       .collect();
    return String::from_utf8(bytes).unwrap();
}

// Change this
pub fn generate_auth_token(user: &User, remember_me: bool) -> String {
    return generate_opaque_token_of_length(32);
}

pub fn save_authentication_token(uuid: Uuid, token: &str) {
    //TODO
}
