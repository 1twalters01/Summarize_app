use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_opaque_token_of_length(length: i64) -> String {
    let mut rng = thread_rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.sample(Alphanumeric)).collect();
    return String::from_utf8(bytes).unwrap();
}
