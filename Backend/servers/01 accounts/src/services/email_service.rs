use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport, Transport,
};
use std::env;

use crate::datatypes::email_types::{MessageType, PasswordResetEmailParams, RegisterEmailParams};

pub struct EmailService<'a> {
    smtp_username: String,
    smtp_password: String,
    smtp_server: String,
    recipient: &'a str,
    subject: Option<String>,
    body: Option<String>,
}

impl<'a> EmailService<'a> {
    pub fn new(recipient: &'a str) -> Self {
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

    pub fn compose_preformatted_message(&mut self, message_type: MessageType) {
        (self.subject, self.body) = match message_type {
            MessageType::RegisterEmail(RegisterEmailParams {
                verification_token,
                register_email_token,
            }) => {
                let subject = String::from("Sign up to Summarize");
                let body = format!(
                    "<h1>Summarize</h1><p>Verification Token: {}</p><p>Register Email Token: {}</p>",
                    verification_token, register_email_token
                );
                (Some(subject), Some(body))
            }
            MessageType::RegisterConfirmation(_) => {
                let subject = String::from("Summarize Registration has been Completed");
                let body =
                    format!("<h1>Summarize</h1><p>Thank you for registering to summarize.</p>");
                (Some(subject), Some(body))
            }
            MessageType::PasswordResetEmail(PasswordResetEmailParams {
                verification_token,
                password_reset_email_token,
            }) => {
                let subject = String::from("Summarize Password Reset");
                let body = format!(
                    "<h1>Summarize</h1><p>Verification Token: {}</p><p>Password Reset Email Token: {}</p>",
                    verification_token, password_reset_email_token
                );
                (Some(subject), Some(body))
            }
            MessageType::PasswordResetConfirmation(_) => {
                let subject = String::from("Summarize Password Reset has been Completed");
                let body =
                    format!("<h1>Summarize</h1><p>Thank you for registering to summarize.</p>");
                (Some(subject), Some(body))
            }
        };
    }

    pub fn send_email(&self) -> Result<(), String> {
        if let (Some(subject), Some(body)) = (self.subject.clone(), self.body.clone()) {
            let mailer = SmtpTransport::relay(&self.smtp_server)
                .unwrap()
                .credentials(Credentials::new(
                    self.smtp_username.clone(),
                    self.smtp_password.clone(),
                ))
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
            return Err("Nothing to send".to_string());
        }
    }
}
