use actix_web::{post, HttpRequest, HttpResponse, Responder, Result, web::Json};

use crate::accounts::{
    datatypes::{
        users::User,
        token_object::UserRememberMe
    },
    db_queries::{
        get_user_from_email_in_pg_users_table,
        set_key_value_in_redis,
        set_token_user_in_redis,
        get_user_from_token_in_redis,
        set_token_tokenObject_in_redis,
        delete_token_in_redis,
        get_user_remember_me_from_token_in_redis,
        get_email_from_token_struct_in_redis,
        create_new_user_in_pg_users_table,
        update_password_for_user_in_pg_users_table
    },
    emails::{
        compose_register_email_message,
        compose_password_reset_email_message,
        send_email,
    },
    schema::{
        AccountError,
        LoginEmailRequestSchema, LoginEmailResponseSchema,
        LoginPasswordRequest, LoginPasswordRequestSchema, LoginPasswordResponseSchema,
        LoginTotpRequest, LoginTotpRequestSchema, LoginTotpResponseSchema,
        RegisterEmailRequestSchema, RegisterEmailToken, RegisterEmailResponseSchema,
        RegisterVerifyRequest, RegisterVerifyRequestSchema, RegisterVerifyResponseSchema,
        RegisterDetailsRequest, RegisterDetailsResponseSchema, 
        PasswordResetRequestSchema, PasswordResetResponseSchema, 
        PasswordResetConfirmRequestSchema, PasswordResetConfirmResponseSchema,
    },
    validations::{
        validate_email, validate_username,
        validate_password, validate_totp,
        validate_first_name, validate_last_name
    },
};
use crate::databases::connections::{
    create_pg_pool_connection,
    create_redis_client_connection
};
use crate::tokens::{
    generate_opaque_token_of_length,
    generate_auth_token,
    save_authentication_token,
};


#[post("register/email")]
async fn register_email(req_body: Json<RegisterEmailRequestSchema>) -> Result<impl Responder> {
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
    let user_result: Result<User, sqlx::Error> = get_user_from_email_in_pg_users_table(&pool, &email).await;

    // if email exists then return an error
    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == true {
        res_body.is_email_stored = true;
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("user already exists")) };
        return Ok(HttpResponse::Conflict() // change to real method - currently have no lsp
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }


    // create a verify token, a register email token, and a register_email_token_struct
    let verification_token = generate_opaque_token_of_length(8);
    let register_email_token = generate_opaque_token_of_length(64);
    let token_struct: RegisterEmailToken = RegisterEmailToken {
        register_email_token: register_email_token.clone(),
        verification_token: verification_token.clone()
    };
    let token_struct_json: String = serde_json::to_string(&token_struct).unwrap();

    // try to email the account a message containing the token
    let message = compose_register_email_message(&verification_token, &register_email_token);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("unable to send an email to this address"))};
        res_body.account_error = error;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // save {key: token, value: email} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token_struct_json, &email, &expiry_in_seconds).await;

    // if redis fails then return an error
    if set_redis_result.is_err() {
        println!("{:?}", set_redis_result);
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // return ok
    res_body.register_response_token = Some(register_email_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}



#[post("register/verify")]
async fn register_verify(req_body: Json<RegisterVerifyRequest>, req: HttpRequest) -> Result<impl Responder> {
    let RegisterVerifyRequest { verification_token } = req_body.into_inner();
    let register_email_token: String = req.headers().get("register_email_token").unwrap().to_str().unwrap().to_string();
    register_verification_functionality(register_email_token, verification_token).await
}



#[post("register/verify/{register_email_token}/{verification_token}")]
async fn register_verify_link(path: actix_web::web::Path<RegisterVerifyRequestSchema>) -> Result<impl Responder> {
    let RegisterVerifyRequestSchema { register_email_token, verification_token } = path.into_inner(); 

    register_verification_functionality(register_email_token, verification_token).await
}

async fn register_verification_functionality(register_email_token: String, verification_token: String) -> Result<impl Responder> {
    let mut res_body: RegisterVerifyResponseSchema = RegisterVerifyResponseSchema::new();

    // Form RegisterToken struct
    let token_struct: RegisterEmailToken = RegisterEmailToken {
        verification_token,
        register_email_token,
    };
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(con, &token_struct_json) {
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
        Ok(email) => email,
    };


    // create a new token
    let register_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(con, &register_verification_token, &email, &expiry_in_seconds);
    if set_redis_result.await.is_err() { panic!("redis error, panic debug") }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_token_in_redis(con, &token_struct_json);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // return ok
    res_body.is_verification_token_correct = true;
    res_body.register_verification_token = Some(register_verification_token);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}


#[post("register/details/{uidb64}/{token}")]
async fn register_details(req_body: Json<RegisterDetailsRequest>, req: HttpRequest) -> Result<impl Responder> {
    let RegisterDetailsRequest { username, password, password_confirmation, first_name, last_name } = req_body.into_inner();
    let register_verification_token: String = req.headers().get("register_email_token").unwrap().to_str().unwrap().to_string();
    let mut res_body: RegisterDetailsResponseSchema = RegisterDetailsResponseSchema::new();

    // get the email from redis using the token
    let con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(con, &register_verification_token) {
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
        Ok(email) => email,
    };

    // check if the username is already found in the database. If it is then return error
    let validated_username = validate_username(username.clone());
    if validated_username.is_err() {
        res_body.account_error = AccountError { is_error: false, error_message: Some(String::from("invalid username"))};
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    if password != password_confirmation {
        res_body.account_error = AccountError { is_error: false, error_message: Some(String::from("password does not match confirmation password"))};
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        res_body.account_error = AccountError { is_error: false, error_message: Some(String::from("invalid password"))};
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body))
    }
    println!("password: {:#?}", password);


    if first_name.is_some() {
        let validated_first_name = validate_first_name(first_name.clone().unwrap());
        if validated_first_name.is_err() {
            res_body.account_error = AccountError { is_error: false, error_message: Some(String::from("invalid first name"))};
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body))
        }
    }

    if last_name.is_some() {
        let validated_last_name = validate_last_name(last_name.clone().unwrap());
        if validated_last_name.is_err() {
            res_body.account_error = AccountError { is_error: false, error_message: Some(String::from("invalid last name"))};
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body))
        }
    }

    let create_user: Result<User, std::io::Error> = User::new(username, email, password);
    if create_user.is_err() {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("internal error"))};
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    let user: User = create_user.unwrap();

    // save details to the user to postgres
    let pool = create_pg_pool_connection().await;
    let save_user_result: Result<(), sqlx::Error> = create_new_user_in_pg_users_table(&pool, user).await;

    // if error then return error
    if save_user_result.is_err() {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("internal error"))};
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return Ok
    // create an auth token with remember me set to false and send it over as well?
    res_body.success = true;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}


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
    println!("email: {}", &email);
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
async fn login_totp(data: Json<LoginTotpRequest>, req: HttpRequest) -> Result<impl Responder> {
    let login_password_response_token: String = req.headers().get("login_password_response_token").unwrap().to_str().unwrap().to_string();
    let LoginTotpRequest { totp } = data.into_inner();
    let LoginTotpRequestSchema { totp, login_password_response_token } = LoginTotpRequestSchema { totp, login_password_response_token };
    let mut res_body: LoginTotpResponseSchema = LoginTotpResponseSchema::new();

    // Try to get TokenObject from redis
    let mut con = create_redis_client_connection();
    let (mut user, remember_me): (User, bool) = match get_user_remember_me_from_token_in_redis(con, &login_password_response_token) {
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
    let validated_totp = validate_totp(totp.clone());
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
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Incorrect totp")) };

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

    // get and serialize user
    let user: User = user_result.unwrap();
    // should serialize just the uuid instead?
    let user_json: String = serde_json::to_string(&user).unwrap();

    // create a token
    let password_reset_response_token: String = generate_opaque_token_of_length(25);

    // try to email the account a message containing the token
    let message = compose_password_reset_email_message(&password_reset_response_token, &user);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        let error: AccountError = AccountError { is_error: true, error_message: Some(String::from("unable to send email"))};
        res_body.account_error = error;
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // add {key: token, value: UUID} to redis
    let con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(300);

    let set_redis_result = set_token_user_in_redis(con, &password_reset_response_token, &user_json, &expiry_in_seconds);
    
    // if redis fails then return an error
    if set_redis_result.await.is_err() {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("Server error")) };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }

    // return ok
    res_body.success = true;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}



#[post("password-reset/{uidb64}/{token}")]
async fn password_reset_confirm(req_body: Json<PasswordResetConfirmRequestSchema>, req: HttpRequest) -> Result<impl Responder> {
    let password_reset_response_email_token: String = req.headers().get("password_reset_response_token").unwrap().to_str().unwrap().to_string();
    let PasswordResetConfirmRequestSchema { password, password_confirmation } = req_body.into_inner();
    let mut res_body: PasswordResetConfirmResponseSchema = PasswordResetConfirmResponseSchema::new();

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

    // get user from token in redis
    let con = create_redis_client_connection();
    let mut user: User = match get_user_from_token_in_redis(con, &password_reset_response_email_token) {
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
        Ok(email) => email,
    };

    // if change is not allowed then error
    user.set_password(password);

    // save change in postgres
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> = update_password_for_user_in_pg_users_table(&pool, &user).await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.account_error = AccountError { is_error: true, error_message: Some(String::from("internal error")) };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body)
        )
    }
    

    // return success
    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(true));
}


