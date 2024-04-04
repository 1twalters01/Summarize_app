use actix_web::{post, HttpResponse, Responder, Result, web::Json};
use crate::accounts::schema::{
    LoginError,
    LoginEmail, LoginEmailResponse,
    LoginPassword, LoginPasswordResponse,
    LoginTotp, LoginTotpResponse,
    RegisterEmail, RegisterVerify, RegisterDetails, 
    UsernameReset, 
    UsernameResetConfirm, 
    PasswordReset, 
    PasswordResetConfirm
};
use crate::accounts::validations::{validate_email, validate_password, validate_totp, validate_username, validate_first_name, validate_last_name};
use crate::databases::connections::{create_pg_pool_connection, create_redis_client_connection};
use crate::accounts::db_queries::{
    fake_postgres_check_email, 
    get_user_from_email_in_pg_users_table
};

#[post("login/email")]
async fn login_email(req_body: Json<LoginEmail>) -> Result<impl Responder> {
    let email: String = req_body.into_inner().email;
    let mut res_body: LoginEmailResponse = LoginEmailResponse::new();

    // Validate the email from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        let error: LoginError = LoginError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.login_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // replace with postgres function
    let is_email_stored = fake_postgres_check_email(&email);
    if is_email_stored == false {
        res_body.is_email_stored = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    
    res_body.is_email_stored = true;
    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(res_body)
    )
}


#[post("login/password")]
async fn login_password(req_body: Json<LoginPassword>) -> Result<impl Responder> {
    let LoginPassword { email, password } = req_body.into_inner();
    let mut res_body: LoginPasswordResponse = LoginPasswordResponse::new();

    // Validate the email and password from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        let error: LoginError = LoginError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.login_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    println!("email: {:#?}", email);

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        let error: LoginError = LoginError{
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.login_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("password: {:#?}", password);

    // perform database query for if the email has an associated account
    // replace with postgres function
    let is_email_stored = fake_postgres_check_email(&email);
    if is_email_stored == false {
        res_body.password_content.is_email_stored = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // do test for if username password is the same as the inputted password
    let is_correct_password = fake_postgres_check_password(&password);
    if is_correct_password == false {
        res_body.password_content.is_password_correct = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // see if account has a totp
    let has_totp = fake_postgres_check_totp(&email);
    if has_totp == true {
        res_body.password_content.has_totp = true;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    let token: String = generate_token();
    res_body.password_content.is_email_stored = true;
    res_body.password_content.is_password_correct = true;
    res_body.password_content.has_totp = false;
    res_body.password_content.token = Some(token);
    save_authentication_token(uid, token);

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body)
    )
}


#[post("login/totp")]
async fn login_totp(req_body: Json<LoginTotp>) -> Result<impl Responder> {
    let LoginTotp { email, password, totp } = req_body.into_inner();
    let mut res_body: LoginTotpResponse = LoginTotpResponse::new();

    // Validate the email and password from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        let error: LoginError = LoginError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.login_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    println!("email: {:#?}", email);

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        let error: LoginError = LoginError{
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.login_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("password: {:#?}", password);

    let validated_totp = validate_totp(totp.clone());
    if validated_totp.is_err() {
        let error: LoginError = LoginError{
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.login_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("password: {:#?}", totp);
 
    // perform database query for if the email has an associated account
    // replace with postgres function
    let is_email_stored = fake_postgres_check_email(&email);
    if is_email_stored == false {
        res_body.totp_content.is_email_stored = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // do test for if username password is the same as the inputted password
    let is_correct_password = fake_postgres_check_password(&password);
    if is_correct_password == false {
        res_body.totp_content.is_password_correct = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // see if account has a totp
    let has_totp = fake_postgres_check_totp(&email);
    if has_totp == true {
        res_body.totp_content.has_totp = true;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // check totp
    let totp_string = generate_fake_totp_string();
    let stored_totp: String = get_totp_from_totp_string(totp_string);
    if stored_totp == totp {
        res_body.totp_content.is_totp_correct = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    res_body.totp_content.is_email_stored = true;
    res_body.totp_content.is_password_correct = true;
    res_body.totp_content.has_totp = true;
    res_body.totp_content.is_totp_correct = true;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
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

