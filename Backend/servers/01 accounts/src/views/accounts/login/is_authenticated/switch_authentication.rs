pub fn post_authentication_options() -> Result<impl Responder> {
    // get uuid
    // get main token
    // get selected method
        // if method is biometrics then get device_id and platform_id
    // check if selected method is active for user (and device_id and platform_id for biometrics)
    // if not then error

    // create new main token and save the user_uuid and remember_me
    // if selected method is biometrics
        // create challenge and challenge token
        // create request_field
    // if selected method is sms
        // generate otp
        // save otp
        // send text with otp
        // create request_field
    // if selected method is totp
        // create request_field

    // send response

}