use actix_web::{post, HttpRequest, HttpResponse, Responder, Result, web::Json};
use crate::accounts::datatypes::{
    users::User,
    token_object::UserRememberMe,
};
use crate::accounts::schema::{
    AccountError,
    LoginEmailRequestSchema, LoginEmailResponseSchema,
    LoginPasswordRequest, LoginPasswordRequestSchema, LoginPasswordResponseSchema,
    LoginTotpRequestSchema, LoginTotpResponseSchema,
    RegisterEmailRequestSchema, RegisterEmailResponseSchema,
    RegisterVerifyRequestSchema, RegisterVerifyResponseSchema,
    RegisterDetailsRequestSchema, RegisterDetailsResponseSchema, 
    PasswordResetRequestSchema, PasswordResetResponseSchema, 
    PasswordResetConfirmRequestSchema, PasswordResetConfirmResponseSchema,
};
use crate::accounts::validations::{validate_email, validate_password, validate_totp, validate_username, validate_first_name, validate_last_name};
use crate::databases::connections::{create_pg_pool_connection, create_redis_client_connection};
use crate::accounts::db_queries::{
    get_user_from_email_in_pg_users_table,
    set_token_user_in_redis,
    get_user_from_token_in_redis,
    set_token_tokenObject_in_redis,
    delete_token_in_redis,
};
use crate::tokens::{
    generate_opaque_token_of_length,
    generate_auth_token,
    save_authentication_token,
};

#[post("login/email")]
async fn login_email(data: Json<LoginEmailRequestSchema>) -> Result<impl Responder> {
    let LoginEmailRequestSchema { email } = data.into_inner();
    let mut res_body: LoginEmailResponseSchema = LoginEmailResponseSchema::new();

    // Validate the email from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        res_body.account_error = AccountError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // try to get the user from postgres using the email
    let pool = create_pg_pool_connection().await;
    let user_result: Result<User, sqlx::Error> = get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;

    // if user does not exist then return an error
    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == false {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("user does not exist")) };
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // set is_email_stored field to true
    res_body.is_email_stored = true;
    
    // create a token
    let token: String = generate_opaque_token_of_length(25);

    // serialize the user
    let user: User = user_result.ok().unwrap();
    let user_json = serde_json::to_string(&user).unwrap();

    // save {key: token, value: user} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_token_user_in_redis(con, &token, &user_json, &expiry_in_seconds);
    
    // if redis fails then return an error
    if set_redis_result.await.is_err() {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    
    // return success
    res_body.is_email_stored = true;
    res_body.login_email_response_token = Some(token);
    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(res_body)
    )
}


#[post("login/password")]
async fn login_password(data: Json<LoginPasswordRequest>, req: HttpRequest) -> Result<impl Responder> {
    let login_email_response_token: String = req.headers().get("login_email_response_token").unwrap().to_str().unwrap().to_string();
    let LoginPasswordRequest { password, remember_me } = data.into_inner();
    let LoginPasswordRequestSchema { login_email_response_token, password, remember_me } = LoginPasswordRequestSchema {
        login_email_response_token,
        password,
        remember_me,
    };
    let mut res_body: LoginPasswordResponseSchema = LoginPasswordResponseSchema::new();

    // try to get user from token in redis
    let con = create_redis_client_connection();
    let user: User = match get_user_from_token_in_redis(con, &login_email_response_token) {
        // if error return error
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
    let check_password: Result<(), std::io::Error> = user.check_password(&password);
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
        // create a token and a serialized UserRememberMe{ remember_me: bool, token: String }
        let token: String = generate_opaque_token_of_length(25);
        let token_object: UserRememberMe = UserRememberMe { remember_me, user };
        let token_object_json = serde_json::to_string(&token_object).unwrap();

        // save {key: token, value: UserRememberMe} to redis
        let expiry_in_seconds: Option<i64> = Some(300);
        let mut con = create_redis_client_connection();
        let set_redis_result = set_token_tokenObject_in_redis(con, &token, &token_object_json, &expiry_in_seconds);
    
        // if redis fails then return an error
        if set_redis_result.await.is_err() {
            res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/json; charset=utf-8")
                .json(res_body)
            )
        }
        
        // delete old token
        con = create_redis_client_connection();
        let delete_redis_result = delete_token_in_redis(con, &login_email_response_token);

        // if redis fails then return an error
        if delete_redis_result.await.is_err() {
            res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/json; charset=utf-8")
                .json(res_body)
            )
        }

        // return success
        res_body.has_totp = true;
        res_body.login_password_response_token = Some(token);
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // generate token
    let token: String = generate_auth_token(&user, remember_me);
    
    // save auth token (use jwt intead?
    save_authentication_token(user.get_uuid(), &token);

    // return success
    res_body.has_totp = false;
    res_body.auth_token = Some(token);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body)
    )
}


        
#[post("login/totp")]
async fn login_totp(data: Json<LoginTotpRequestSchema>, req: HttpRequest) -> Result<impl Responder> {
    let login_password_response_token: String = req.headers().get("login_password_response_token").unwrap().to_str().unwrap().to_string();
    let LoginTotpRequest { totp } = data.into_inner();
    let LoginTotpRequestSchema { totp, login_password_response_token } = LoginTotpRequestschema { totp, login_password_response_token };
    let mut res_body: LoginTotpResponseSchema = LoginTotpResponseSchema::new();

    // Try to get TokenObject from redis
    let mut con = create_redis_client_connection();
    let user: UserRememberMe = get_userRememberMe_from_token_in_redis(con, &login_password_response_token).unwrap();
    let (user, remember_me): (User, bool) = match get_userRememberMe_from_token_in_redis(con, &login_password_response_token) {
        // if error return error
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
        Ok(user_remember_me) => (user_remember_me.user, user_remember_me.remember_me),
    };

    // check if the entered totp is a valid totp
    let validated_totp = validate_totp(password.clone());
    if validated_totp.is_err() {
        let error: AccountError = AccountError{
            is_error: true,
            error_message: Some(validated_totp.err().unwrap())
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("totp: {:#?}", totp);

    // see if account has a totp
    let has_totp = user.is_totp_activated();
    if has_totp == false {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("User does not have totp activated")) };
        res_body.account_error = error;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // check totp
    let is_totp_correct = user.check_totp(totp);
    if is_totp_correct == false {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("Incorrect totp")) };
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // delete old token from redis
        con = create_redis_client_connection();
        let delete_redis_result = delete_token_in_redis(con, &login_password_response_token);

        // if redis fails then return an error
        if delete_redis_result.await.is_err() {
            res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/json; charset=utf-8")
                .json(res_body)
            )
        }

    // create auth token
    let token: String = generate_auth_token(&user, remember_me);

    // save auth token (use jwt intead?
    save_authentication_token(user.get_uuid(), &token);

    // return success
    res_body.is_totp_correct = true;
    res_body.auth_token = Some(token);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}


#[post("register/email")]
async fn registerEmail(req_body: Json<RegisterEmailRequestSchema>) -> Result<impl Responder> {
    let RegisterEmailRequestSchema { email } = req_body.into_inner();
    let mut res_body: RegisterEmailResponseSchema = RegisterEmailResponseSchema::new();

    // Validate the email from the request body
    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        res_body.account_error = AccountError{
            is_error: true,
            error_message: Some(validated_email.err().unwrap())
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // try to get the user from postgres using the email
    let pool = create_pg_pool_connection().await;
    let user_result: Result<User, sqlx::Error> = get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;

    // if user exists then return an error
    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == false {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("user does not exist")) };
        return Ok(HttpResponse::Conflict() // change to real method - currently have no lsp
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }




    
    // create a token
    let token = generate_opaque_token_of_length(25);

    // try to email the account a message containing the token
    let message_result: bool = compose_registerEmail_email(&token);

    // if unable to email then return an error
    if message_result = false {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("email not found"))};
        res_body.account_error = error;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // add {key: token, value: email} to redis
    let con = create_redis_client_connection();
    let user: User = user_result.ok().unwrap();
    let expiry_in_seconds: Option<i64> = Some(300);

    let set_redis_result = set_token_email_in_redis(con, &token, &email, &expiry_in_seconds);
    
    if set_redis_result.await.is_err() { panic!("redis error, panic debug") }
    
    // return ok
    res_body.is_email_stored = true;
    res_body.register_response_token = Some(token);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("register/verify/{uidb64}/{token}")]
async fn registerVerify(req_body: Json<RegisterVerifyRequestSchema>) -> Result<impl Responder> {
    let RegisterVerifyRequestSchema { register_response_token, verification_token } = req_body.into_inner();
    let mut res_body: RegisterVerifyResponseSchema = RegisterVerifyResponseSchema::new();
    
    // Get email from token using redis
    
    // If no result or wrong format then return error
    
    // Check if email is in postgres database
    
    // if in database then return some conflict error
    
    // create temporary user in postgres with blank details for 5 mins?


    // create a token
    let token = generate_opaque_token_of_length(25);

    // add {key: token, value: user postgres UUID} to redis
    let con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(300);
    let set_redis_result = set_token_email_in_redis(con, &token, &email, &expiry_in_seconds);
    if set_redis_result.await.is_err() { panic!("redis error, panic debug") }

    // return ok
    res_body.is_verification_token_correct = true;
    res_body.verify_response_token = Some(token);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
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
async fn password_reset(req_body: Json<PasswordResetRequestSchema>) -> Result<impl Responder> {
    let PasswordResetRequestSchema { email } = req_body.into_inner();
    let mut res_body: PasswordResetResponseSchema = PasswordResetResponseSchema::new();

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()))
    }

    // Check if email is in postgres database
    let pool = create_pg_pool_connection().await;
    let user_result: Result<User, sqlx::Error> = get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;

    // if not in database then return some not found error
    if user_result.is_ok() == false {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("email not found"))};
        res_body.account_error = error;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    // create a token
    let token = generate_opaque_token_of_length(25);

    // try to email the account a message containing the token
    let message_result: bool = compose_passwordResetEmail_email(&token);

    // if unable to email then return an error
    if message_result = false {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("email not found"))};
        res_body.account_error = error;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // add {key: token, value: UUID} to redis
    let con = create_redis_client_connection();
    let user: User = user_result.ok().unwrap();
    let expiry_in_seconds: Option<i64> = Some(300);

    let set_redis_result = set_token_userUUID_in_redis(con, &token, &user.get_uuid(), &expiry_in_seconds);
    
    if set_redis_result.await.is_err() { panic!("redis error, panic debug") }
    // return ok
    res_body.is_email_stored = true;
    res_body.register_response_token = Some(token);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("password-reset/{uidb64}/{token}")]
async fn password_reset_confirm(req_body: Json<PasswordResetConfirmRequestSchema>) -> Result<impl Responder> {
    let PasswordResetConfirmResponseSchema { password, password_confirmation } = req_body.into_inner();

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

    // get uuid from uidb64
    let uuid = 
    // if change is not allowed then error
    // set username to the username

    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(true));
}


