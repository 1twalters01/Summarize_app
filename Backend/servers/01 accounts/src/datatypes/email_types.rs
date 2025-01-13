pub enum MessageType {
    RegisterEmail(RegisterEmailParams),
    // RegisterConfirmation(RegisterConfirmationParams),
    PasswordResetEmail(PasswordResetEmailParams),
    // PasswordResetConfirmation(PasswordResetConfirmationParamas),
    // NewLoginNotification(NewLoginNotificationParams),
}

pub struct RegisterEmailParams {
    verification_token: &str,
    register_email_token: &str,
}

pub struct RegisterConfirmationParams {
    verification_token: &str,
    password_reset_email_token: &str,
}