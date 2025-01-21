use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport, Transport,
};
use std::env;

use crate::datatypes::email_types::{
    MessageType,
    RegisterEmailParams, PasswordResetEmailParams
};

pub struct EmailService{
    smtp_username: String,
    smtp_password: String,
    smtp_server: String,
    recipient: String,
    subject: Option<String>,
    body: Option<String>,
}

impl EmailService {
    pub fn new(recipient: String) -> Self {
        let smtp_username: String = env::var("SMTP_USERNAME").unwrap();
        let smtp_password: String = env::var("SMTP_PASSWORD").unwrap();
        let smtp_server: String = env::var("SMTP_SERVER").unwrap();

        EmailService {
            smtp_username,
            smtp_password,
            smtp_server,
            recipient,
            subject: None,
            body: None,
        }
    }

    pub fn compose_message(&mut self, message_type: MessageType) {
        (self.subject, self.body) = match message_type {
            MessageType::RegisterEmail(RegisterEmailParams { verification_token, register_email_token }) => {
                let subject = String::from("Sign up to Summarize");
                let body = format!(
                    "<h1>Summarize</h1><p>Verification Token: {}</p><p>Register Email Token: {}</p>",
                    verification_token, register_email_token
                );
                (Some(subject), Some(body))
            },
            MessageType::PasswordResetEmail(PasswordResetEmailParams { verification_token, password_reset_email_token }) => {
                let subject = String::from("Summarize Password Reset");
                let body = format!(
                    "<h1>Summarize</h1><p>Verification Token: {}</p><p>Password Reset Email Token: {}</p>",
                    verification_token, password_reset_email_token
                );
                (Some(subject), Some(body))
            }
        };

        // self.subject = subject;
        // self.body = body;
    } 

    pub fn send_email(&self) -> Result<(), String> {
        if let (Some(subject), Some(body)) = (self.subject.clone(), self.body.clone()) {
            let mailer = SmtpTransport::relay(&self.smtp_server)
                .unwrap()
                .credentials(Credentials::new(self.smtp_username.clone(), self.smtp_password.clone()))
                .authentication(vec![Mechanism::Plain])
                .build();

            let email = Message::builder()
                .from(self.smtp_username.parse().unwrap())
                .to(self.recipient.parse().unwrap())
                .subject(subject)
                .header(ContentType::TEXT_HTML)
                .body(body)
                .unwrap();

            match mailer.send(&email) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(err.to_string()),
            }
        } else {
            return Err("Nothing to send".to_string())
        }
    }
}
