use std::env;

pub struct TextService<'a> {
    lgoin_info: String, // Change to something real
    recipient: &'a str,
    message: Option<String>,
}

impl<'a> TextService<'a> {
    pub fn new(recipient: &'a str) -> Self {
        let login_info: String = env::var("LOGIN_INFO").unwrap(); // Change to something real

        TextService {
            login_info,
            recipient,
            message: None,
        }
    }

    pub fn compose_preformatted_message(&mut self, message_type: MessageType) {
        (self.message) = match message_type {
            MessageType::LoginSms(LoginSmsParams {otp}) => {
                let message = format!(
                    "<h1>Summarize</h1><p>Your OTP: {}</p>",
                    otp,
                );
                (Some(message))
            }
        }
    }

    pub fn send_text(&self) -> Result<(), String> {
        if let (Some(message)) = (self.body.clone()) {
        } else {
            return Err("Nothing to send".to_string());
        }
    }
}