use std::env;

use oauth2::basic::BasicClient

use crate::datatypes::oauth_types::ClientProvider;
pub struct OAuthService;


impl OAuthService {
    pub fn new_basic_client(client_provider: ClientProvider) -> BasicClient {
        match client_provider {
            Apple => {
                let apple_client_id = ClientId::new(env::var("APPLE_CLIENT_ID").unwrap());
                let apple_client_secret = ClientSecret::new(env::var("APPLE_CLIENT_SECRET").unwrap());
                let apple_auth_url = AuthUrl::new(env::var("APPLE_AUTH_URL").unwrap()).unwrap();
                let apple_token_url = TokenUrl::new(env::var("APPLE_TOKEN_URL").unwrap()).unwrap();
                let client = BasicClient::new(apple_client_id)
                    .set_client_secret(apple_client_secret)
                    .set_auth_uri(apple_auth_url)
                    .set_token_uri(apple_token_url)
                    .set_redirect_uri(RedirectUrl::new("redirect url".to_string()).unwrap());
            },
            Google => {
                let google_client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap());
                let google_client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap());
                let google_auth_url = AuthUrl::new(env::var("GOOGLE_AUTH_URL").unwrap()).unwrap();
                let google_token_url = TokenUrl::new(env::var("GOOGLE_TOKEN_URL").unwrap()).unwrap();
                
                BasicClient::new(google_client_id)
                    .set_client_secret(google_client_secret)
                    .set_auth_uri(google_auth_url)
                    .set_token_uri(google_token_url)
                    .set_redirect_uri(RedirectUrl::new("redirect url".to_string()).unwrap())
            }
        }
    }
}