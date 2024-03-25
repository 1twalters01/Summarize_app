use actix_web::{post, HttpResponse, Responder, Result, web::Json};
use crate::accounts::datatypes::{
    LoginEmail, LoginPassword, LoginTotp, 
    RegisterEmail, RegisterVerify, RegisterDetails, 
    UsernameReset, 
    UsernameResetConfirm, 
    PasswordReset, 
    PasswordResetConfirm};
use crate::validations::{validate_email, validate_password, validate_totp, validate_username, validate_first_name, validate_last_name};

#[post("login/email")]
async fn login_email(req_body: Json<LoginEmail>) -> Result<impl Responder> {
    let email: String = req_body.into_inner().email;

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    let email_database = vec![
        String::from("test@something.com"),
        String::from("test2@something.com"),
        String::from("test3@something.com")];

    if email_database.contains(&email) {
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(true))
    } else {
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .json(email))
    }
}

#[post("login/password")]
async fn login_password(req_body: Json<LoginPassword>) -> Result<impl Responder> {
    let LoginPassword { email, password } = req_body.into_inner();

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()))
    }
    println!("password: {:#?}", password);

    // perform database query for if the email has an associated account
    // if false return false
    // see if account has a totp
    // if true return totp
    // do test for if username password is the same as the inputted password
    // return true or false

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json([email, password]))
}

#[post("login/totp")]
async fn login_totp(req_body: Json<LoginTotp>) -> Result<impl Responder> {
    let LoginTotp { email, password, totp } = req_body.into_inner();

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()))
    }
    println!("password: {:#?}", password);

    let validated_totp = validate_totp(totp.clone());
    if validated_totp.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_totp.err().unwrap()))
    }
    println!("password: {:#?}", totp);
 
    // validate email
    // validate password
    // validate totp
    // if any invalid then error
    // perform database query for if the email has an associated account
    // if false return false
    // see if account has a totp
    // if false return error
    // do test for if username password is the same as the inputted password
    // if false return error
    // check totp
    // if false return error
    // return true or false

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(email))
}

#[post("register/email")]
async fn registerEmail(req_body: Json<RegisterEmail>) -> Result<impl Responder> {
    let email: String = req_body.into_inner().email;

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    let email_database = vec![
        String::from("test@something.com"),
        String::from("test2@something.com"),
        String::from("test3@something.com")];

    if email_database.contains(&email) {
        return Ok(HttpResponse::Conflict()
            .content_type("application/json; charset=utf-8")
            .json(true))
    } else {
        // Create user and add to database
        // Create token instance, associate it with uid and add to token database
        // encode the uid to get a uidb64
        // combine the uid and the token
        // email the user with the token
        // if error return failed
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(true))
    }
}

#[post("register/verify/{uidb64}/{token}")]
async fn registerVerify(req_body: Json<RegisterVerify>) -> Result<impl Responder> {
    let RegisterVerify { email, token } = req_body.into_inner();
    // let uid = base64::decode(uidb64).unwrap();

    // check token associated with uid
    let stored_token = "stored token from database";

    if stored_token == token {
        // set user.active to true
        // return true
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(true))
    }

    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(false))
}

#[post("register/details/{uidb64}/{token}")]
async fn registerDetails(req_body: Json<RegisterDetails>) -> Result<impl Responder> {
    let RegisterDetails { email, password, password_confirmation, username, first_name, last_name, token } = req_body.into_inner();

    // use token to get associated uid

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    if password != password_confirmation {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(false))
    }

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()))
    }
    println!("password: {:#?}", password);

    let validated_username = validate_username(username.clone());
    if validated_username.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_username.err().unwrap()))
    }
    // check if the username is already found in the database. If it is then return error

    if first_name.is_some() {
        let validated_first_name = validate_first_name(first_name.clone().unwrap());
        if validated_first_name.is_err() {
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(validated_first_name.err().unwrap()))
        }
    }

    if last_name.is_some() {
        let validated_last_name = validate_last_name(last_name.clone().unwrap());
        if validated_last_name.is_err() {
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(validated_last_name.err().unwrap()))
        }
    }

    // save details to the user

    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(false))
}

#[post("username-reset")]
async fn username_reset(req_body: Json<UsernameReset>) -> Result<impl Responder> {
    let UsernameReset { email } = req_body.into_inner();

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    let email_database = vec![
        String::from("test@something.com"),
        String::from("test2@something.com"),
        String::from("test3@something.com")];

    if email_database.contains(&email) {
        // generate uidb64
        // generate token
        // create link
        // send email that contains link
        // set allow change username to true
        return Ok(HttpResponse::Conflict()
            .content_type("application/json; charset=utf-8")
            .json(true))
    } else {
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .json(false))
    }
}

#[post("username-reset/{uidb64}/{token}")]
async fn username_reset_confirm(req_body: Json<UsernameResetConfirm>) -> Result<impl Responder> {
    let UsernameResetConfirm { username } = req_body.into_inner();

    let validated_username = validate_username(username.clone());
    if validated_username.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_username.err().unwrap()))
    }

    // check if the username is already found in the database. If it is then return error
    // get uid from uidb64
    // if change is not allowed then error
    // set username to the username

    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(true))
}

#[post("password-reset")]
async fn password_reset(req_body: Json<PasswordReset>) -> Result<impl Responder> {
    let PasswordReset { email } = req_body.into_inner();

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    let email_database = vec![
        String::from("test@something.com"),
        String::from("test2@something.com"),
        String::from("test3@something.com")];

    if email_database.contains(&email) {
        // generate uidb64
        // generate token
        // create link
        // send email that contains link
        // set allow change password to true
        return Ok(HttpResponse::Conflict()
            .content_type("application/json; charset=utf-8")
            .json(true))
    } else {
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .json(false))
    }
}

#[post("password-reset/{uidb64}/{token}")]
async fn password_reset_confirm(req_body: Json<PasswordResetConfirm>) -> Result<impl Responder> {
    let PasswordResetConfirm { password, password_confirmation } = req_body.into_inner();

    if password != password_confirmation {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(false))
    }

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()))
    }

    // get uid from uidb64
    // if change is not allowed then error
    // set username to the username

    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(true))
}


