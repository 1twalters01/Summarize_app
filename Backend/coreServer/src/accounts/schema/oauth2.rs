use actix_web::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

pub struct OAuth2Client {
    client: Client,
    auth_url: String,
    token_url: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    // jwt_secret: String,
}

impl OAuth2Client {
    pub fn from(
        auth_url: &str,
        token_url: &str,
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
        // jwt_secret: &str,
    ) -> Self {
        OAuth2Client {
            client: Client::new(),
            auth_url: auth_url.to_string(),
            token_url: token_url.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            // jwt_secret: jwt_secret.to_string(),
        }
    }

    pub fn new_google() -> Self {
        // Google - https://accounts.google.com/o/oauth2/v2/auth
        let auth_url = env::var("").unwrap();
        let token_url = env::var("").unwrap();
        let client_id = env::var("").unwrap();
        let client_secret = env::var("").unwrap();
        let redirect_uri = env::var("").unwrap();
        // let jwt_secret = env::var("").unwrap();

        let oauth_client: OAuth2Client = OAuth2Client::from(
            &auth_url,
            &token_url,
            &client_id,
            &client_secret,
            &redirect_uri,
        );

        return oauth_client;
    }

    pub fn get_authorization_url(&self, state: &str, scope: &str) -> String {
        let mut url = Url::parse(&self.auth_url).unwrap();
        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("scope", scope)
            .append_pair("state", state);

        return url.to_string();
    }

    pub async fn get_access_tokens_from_code(
        &self,
        code: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let params = TokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: code.to_string(),
            redirect_uri: self.redirect_uri.clone(),
            grant_type: "authorization_code".to_string(),
        };

        let response = self
            .client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response = response.json::<TokenResponse>().await?;
            Ok(token_response)
        } else {
            Err(response.error_for_status().err().unwrap())
        }
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let params = RefreshTokenRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            refresh_token: refresh_token.to_string(),
            grant_type: "refresh_token".to_string(),
        };

        let response = self
            .client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response = response.json::<TokenResponse>().await?;
            Ok(token_response)
        } else {
            Err(response.error_for_status().err().unwrap())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    grant_type: String,
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Serialize)]
struct RefreshTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub grant_type: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenQuery {
    pub refresh_token: String,
}
