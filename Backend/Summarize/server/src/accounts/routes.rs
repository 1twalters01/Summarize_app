use actix_web::{post, HttpRequest, HttpResponse, Responder, Result, web::Json};
use crate::accounts::datatypes::users::User;
use crate::accounts::schema::{
    AccountError,
    LoginEmailRequestSchema, LoginEmailResponseSchema,
    LoginPasswordRequestSchema, LoginPasswordResponseSchema,
    LoginTotpRequestSchema, LoginTotpResponseSchema,
    RegisterEmailRequestSchema, RegisterEmailResponseSchema,
    RegisterVerifyRequestSchema, RegisterVerifyResponseSchema,
    RegisterDetailsRequestSchema, RegisterDetailsResponseSchema, 
    PasswordReset, 
    PasswordResetConfirm
};
use crate::accounts::validations::{validate_email, validate_password, validate_totp, validate_username, validate_first_name, validate_last_name};
use crate::databases::connections::{create_pg_pool_connection, create_redis_client_connection};
use crate::accounts::db_queries::{
    get_user_from_email_in_pg_users_table,
    set_token_user_in_redis,
    get_user_from_token_in_redis,
};
use crate::tokens::{
    generate_opaque_token_of_length,
    generate_auth_token,
    save_authentication_token,
};

#[post("login/email")]
async fn login_email(data: Json<LoginEmailRequestSchema>) -> Result<impl Responder> {
    let email: String = data.into_inner().email;
    let mut res_body: LoginEmailResponseSchema = LoginEmailResponseSchema::new();

    // Validate the email from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    let pool = create_pg_pool_connection().await;

    let user_result: Result<User, sqlx::Error> = get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;
    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == false {
        res_body.is_email_stored = false;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    
    let con = create_redis_client_connection();
    let token: String = generate_opaque_token_of_length(25);
    let user: User = user_result.ok().unwrap();
    let expiry_in_seconds: Option<i64> = Some(300);

    let user_json = serde_json::to_string(&user).unwrap();
    let set_redis_result = set_token_user_in_redis(con, &token, &user_json, &expiry_in_seconds);
    
    if set_redis_result.await.is_err() { panic!("redis error, panic debug") }
    
    res_body.is_email_stored = true;
    res_body.login_email_response_token = token;
    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(res_body)
    )
}


#[post("login/password")]
async fn login_password(data: Json<LoginPasswordRequestSchema>, req: HttpRequest) -> Result<impl Responder> {
    let login_email_response_token: String = req.headers().get("login_email_response_token").unwrap().to_str().unwrap().to_string();
    let LoginPasswordRequestSchema { login_email_response_token, password, remember_me } = LoginPasswordRequestSchema {
        login_email_response_token,
        password: data.clone().password,
        remember_me: data.into_inner().remember_me,
    };
    let mut res_body: LoginPasswordResponseSchema = LoginPasswordResponseSchema::new();

    // get user from token in redis
    let con = create_redis_client_connection();
    let user: User = match get_user_from_token_in_redis(con, &login_email_response_token) {
        Err(err) => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(err),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
        },
        Ok(user) => user,
    };

    // check if the entered password is a valid password
    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("password: {:#?}", password);

    

    // check if password is correct for the given user
    let check_password = user.check_password(&password);
    // let is_correct_password = fake_postgres_check_password(&password);
    if check_password.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(String::from("Incorrect password")),
        };

        res_body.account_error = error;
        res_body.is_password_correct = false;
        
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    res_body.is_password_correct = true;

    // see if account has a totp
    if user.is_totp_activated() == true {
        // need to think of how to retain state of remember_me
        // add a 0 or 1 to the end of the token to keep state?
        // create a TokenObject{ totp: true, token: String } as an &str using serde_json?
        let token: String = generate_opaque_token_of_length(25);
        res_body.has_totp = true;
        res_body.login_password_response_token = Some(token);
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    let token: String = generate_auth_token(&user, remember_me);
    save_authentication_token(user.get_uuid(), &token);
    res_body.has_totp = false;
    res_body.auth_token = Some(token);

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body)
    )
}


        
#[post("login/totp")]
async fn login_totp(data: Json<LoginTotpRequestSchema>, req: HttpRequest) -> Result<impl Responder> {
    let LoginTotp { email, password, totp } = data.into_inner();
    let mut res_body: LoginTotpResponse = LoginTotpResponse::new();

    // Validate the email and password from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    println!("email: {:#?}", email);

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("password: {:#?}", password);

    let validated_totp = validate_totp(totp.clone());
    if validated_totp.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(validated_password.err().unwrap())
        };
        res_body.account_error = error;

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
async fn registerEmail(req_body: Json<RegisterEmailRequestSchema>) -> Result<impl Responder> {
    let email: String = req_body.into_inner().email;

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }
    println!("email: {:#?}", email);

    

    // Check if email is in postgres database
    // if in database then return some conflict error
    // create a token
    // try to email the account a message containing the token
    // if unable to email then return an error
    // add {key: token, value: email} to redis
    // return ok
}

#[post("register/verify/{uidb64}/{token}")]
async fn registerVerify(req_body: Json<RegisterVerifyRequestSchema>) -> Result<impl Responder> {
    let token: String = req_body.into_inner();
    
    // Get email from token using redis
    // If no result or wrong format then return error
    // Check if email is in postgres database
    // if in database then return some conflict error
    // create temporary user in postgres with blank details for 5 mins?
    // create a token
    // add {key: token, value: user postgres UUID} to redis
    // return ok
}

#[post("register/details/{uidb64}/{token}")]
async fn registerDetails(req_body: Json<RegisterDetailsRequestSchema>) -> Result<impl Responder> {
    // use token (in header?)to get associated uuid
    // if no result or wrong format then return error
    
    let RegisterDetailsRequestSchema { username, password, password_confirmation, first_name, last_name } = req_body.into_inner();

    // check if the username is already found in the database. If it is then return error

    let validated_username = validate_username(username.clone());
    if validated_username.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_username.err().unwrap()))
    }

    if password != password_confirmation {
        
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json())
    }

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()))
    }
    println!("password: {:#?}", password);

    
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

    // save details to the user to postgres

    
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

    // Check if email is in postgres database
    // if not in database then return some not found error
    // create a token
    // try to email the account a message containing the token
    // if unable to email then return an error
    // add {key: token, value: email} to redis
    // return ok
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


