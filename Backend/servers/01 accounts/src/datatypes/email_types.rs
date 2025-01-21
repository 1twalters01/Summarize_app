pub enum MessageType<'a> {
    RegisterEmail(RegisterEmailParams<'a>),
    // RegisterConfirmation(RegisterConfirmationParams),
    PasswordResetEmail(PasswordResetEmailParams<'a>),
    // PasswordResetConfirmation(PasswordResetConfirmationParamas),
    // NewLoginNotification(NewLoginNotificationParams),
}

pub struct RegisterEmailParams<'a> {
    pub(crate) verification_token: &'a str,
    pub(crate) register_email_token: &'a str,
}

pub struct PasswordResetEmailParams<'a> {
    pub(crate) verification_token: &'a str,
    pub(crate) password_reset_email_token: &'a str,
}
