pub enum MessageType<'a> {
    LoginSms(LoginSmsParams<'a>,)
}

pub struct LoginSmsParams<'a> {
    pub(crate) otp: &[u8; 6],
}