use std::env;

pub struct TextService<'a> {
    lgoin_info: String, // Change to something real
    recipient: &'a str,
    body: Option<String>,
}

impl<'a> TextService<'a> {
    pub fn new(recipient: &'a str) -> Self {
        let login_info: String = env::var("LOGIN_INFO").unwrap(); // Change to something real

        TextService {
            smtp_username,
            smtp_password,
            smtp_server,
            recipient,
            subject: None,
            body: None,
        }
    }

    pub fn compose_preformatted_message(&mut self, message_type: MessageType) {
    }

    pub fn send_email(&self) -> Result<(), String> {
    }
}