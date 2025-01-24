use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use dotenv::dotenv;
use uuid::Uuid;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::login::refresh::response::{
        response::ResponseField, Error, Response,
    },
    services::{response_service::ResponseService, token_service::TokenService},
    utils::validations::refresh_token::validate_refresh_token,
};

pub async fn post_refresh_token(req: HttpRequest) -> Result<impl Responder> {
    dotenv()::ok();

    let opaque_token: String = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(opaque_token) => opaque_token.to_string(),
            Err(err) => {
                println!("err: {}", err);
                return Ok(ResponseService::create_error_response(
                    AppError::LoginRefresh(Error::InvalidRefreshToken),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
        },
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginRefresh(Error::RefreshTokenNotFound),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    if validate_authentication_token(&opaque_token).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginRefresh(Error::InvalidRefreshToken),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let access_token: String;
    let token_prefix: String;
    if opaque_token.starts_with("SITE_") {
        token_prefix = "SITE_";

        // Get refresh token for user

        // try to Get user uuid from refresh token else fail
        let user_uuid: Uuid = match TokenService::get_user_uuid_from_refresh_token(&refresh_token).await
        {
            Ok(Some(user_uuid)) => user_uuid,
            Ok(None) => {
                return Ok(ResponseService::create_error_response(
                    AppError::LoginRefresh(Error::UserNotFound),
                    StatusCode::NOT_FOUND,
                ));
            }
            Err(err) => {
                println!("err: {}", err);
                return Ok(ResponseService::create_error_response(
                    AppError::LoginRefresh(Error::ServerError),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };

        // Create new access token for user
        let token_service = TokenService::from_uuid(&user_uuid);
        access_token = token_service.generate_access_token().unwrap();
    } else {
        if opaque_token.starts_with("GOOGLE_") {
            let google_client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap());
            let google_client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap());
            let google_auth_url = AuthUrl::new(env::var("GOOGLE_AUTH_URL").unwrap()).unwrap();
            let google_token_url = TokenUrl::new(env::var("GOOGLE_TOKEN_URL").unwrap()).unwrap();
            let client = BasicClient::new(google_client_id)
                .set_client_secret(google_client_secret)
                .set_auth_uri(google_auth_url)
                .set_token_uri(google_token_url)
                .set_redirect_uri(
                    RedirectUrl::new("redirect url".to_string()).expect("Invalid redirect URL"),
                );
            token_prefix = "GOOGLE_";
        } else if opaque_token.starts_with("APPLE_") {
            let apple_client_id = ClientId::new(env::var("APPLE_CLIENT_ID").unwrap());
            let apple_client_secret = ClientSecret::new(env::var("APPLE_CLIENT_SECRET").unwrap());
            let apple_auth_url = AuthUrl::new(env::var("APPLE_AUTH_URL").unwrap()).unwrap();
            let apple_token_url = TokenUrl::new(env::var("APPLE_TOKEN_URL").unwrap()).unwrap();
            let client = BasicClient::new(apple_client_id)
                .set_client_secret(apple_client_secret)
                .set_auth_uri(apple_auth_url)
                .set_token_uri(apple_token_url)
                .set_redirect_uri(
                    RedirectUrl::new("redirect url".to_string()).expect("Invalid redirect URL"),
                );
            token_prefix = "APPLE_";
        } else {
            return Ok(ResponseService::create_error_response(
                AppError::LoginRefresh(Error::InvalidRefreshToken),
                StatusCode::UNAUTHORIZED,
            ));
        }

        let http_client = reqwest::blocking::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        // Get refresh token from opaque token

        let token_result = client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request(http_client);

        let (access_token, access_token_expiration) = match token_result {
            Ok(token) => {
                // Extract access and refresh tokens
                let access_token = token.access_token().secret().clone();
                let access_token_expiration = token.expires_in().unwrap_or_default().as_secs();
                (access_token, access_token_expiration)
            },
            Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        }
    }

    // generate opaque token with prefix
    new_opaque_token = token_prefix + &access_token;
    // save: con.set_ex(format!("session:{}", new_opaque_token), access_token, expiration as usize)

    // Return opaque token
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginRefresh(Response {
            response_field: Some(ResponseField::Token(access_token)),
        }),
        StatusCode::OK,
    ));
}
