use chrono::Duration;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RevocationErrorResponseType, RevocationUrl, Scope, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
    TokenResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    url::Url,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct OauthUser {
    pub access_token: String,
    //pub refresh_token: Option<RefreshToken>,
    pub token_type: BasicTokenType,
    pub expires_in: Option<Duration>,
}

pub type GoogleOAuthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointSet,
>;

pub async fn build_oauth() -> Result<GoogleOAuthClient, Box<dyn Error>> {
    let mut redirect_url = Url::parse(&env::var("BASE_URL")?)?;
    redirect_url.set_path("/auth/callback");
    let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID")?);
    let redirect_url = RedirectUrl::from_url(redirect_url);
    let client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET")?);
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    let client = BasicClient::new(client_id)
        .set_redirect_uri(redirect_url)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_revocation_url(RevocationUrl::new(
            "https://oauth2.googleapis.com/revoke".into(),
        )?);

    Ok(client)
}

pub async fn create_oauth(
    client: &GoogleOAuthClient,
) -> Result<(Url, CsrfToken, PkceCodeVerifier), Box<dyn Error>> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/youtube.readonly".into(),
        ))
        .set_pkce_challenge(pkce_challenge)
        .url();

    Ok((auth_url, csrf_token, pkce_verifier))
}

pub async fn get_oauth_token(
    client: &GoogleOAuthClient,
    code: AuthorizationCode,
    pkce_verifier: PkceCodeVerifier,
    http_client: &reqwest::Client,
) -> Result<OauthUser, Box<dyn Error>> {
    let token_response: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType> = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(http_client)
        .await?;
    let access_token = token_response.access_token().secret().to_string();
    //let refresh_token = token_response.refresh_token().cloned();
    let token_type = token_response.token_type().to_owned();
    let expires_in = token_response
        .expires_in()
        .map(|stdur| Duration::from_std(stdur))
        .transpose()?;
    let oauth_user = OauthUser {
        access_token,
        //refresh_token,
        token_type,
        expires_in,
    };
    Ok(oauth_user)
}

pub async fn revoke_token(
    client: &GoogleOAuthClient,
    http_client: &reqwest::Client,
    access_token: AccessToken,
) -> Result<(), Box<dyn Error>> {
    let token_to_revoke = access_token.into();
    client
        .revoke_token(token_to_revoke)?
        .request_async(http_client)
        .await?;
    Ok(())
}
