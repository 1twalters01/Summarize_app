use std::env;

use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport, Transport,
};

#[derive(Debug)]
pub struct EmailMessage {
    pub subject: String,
    pub body: String,
}

pub fn send_email(
    message: EmailMessage,
    email: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    // SMTP server credentials
    let smtp_username: String = env::var("SMTP_USERNAME").unwrap();
    let smtp_password: String = env::var("SMTP_PASSWORD").unwrap();
    let smtp_server: String = env::var("SMTP_SERVER").unwrap();

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(Credentials::new(smtp_username.clone(), smtp_password))
        .authentication(vec![Mechanism::Plain])
        .build();

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(email.parse().unwrap())
        .subject(message.subject)
        .header(ContentType::TEXT_HTML)
        .body(message.body)
        .unwrap();

    match mailer.send(&email) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    }
}
