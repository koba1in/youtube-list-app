use actix_session::Session;
use oauth2::{AccessToken, AuthorizationCode, CsrfToken, PkceCodeVerifier, url::Url};
use reqwest::Client;
use std::error::Error;
use uuid::Uuid;

use crate::outer::{
    googleoauth::{GoogleOAuthClient, create_oauth, get_oauth_token, revoke_token},
    redis::{
        OAuthSession, User, create_oauth_session, create_user, delete_user, get_user, validate_user,
    },
};

pub async fn start_oauth(
    session: Session,
    client: &GoogleOAuthClient,
) -> Result<(Url, Uuid, CsrfToken), Box<dyn Error>> {
    let (auth_url, csrf_token, pkce_verifier) = create_oauth(client).await?;
    let uuid = create_oauth_session(session, csrf_token.clone(), pkce_verifier).await?;
    Ok((auth_url, uuid, csrf_token))
}

pub async fn complete_oauth(
    session: Session,
    client: &GoogleOAuthClient,
    http_client: &Client,
    uuid: Uuid,
    csrf_token: CsrfToken,
    code: AuthorizationCode,
) -> Result<CsrfToken, Box<dyn Error>> {
    let user: Option<OAuthSession> =
        if validate_user::<OAuthSession>(session.clone(), uuid, csrf_token).await {
            get_user::<OAuthSession>(session.clone(), uuid).await?
        } else {
            None
        };
    if let Some(user) = user {
        let new_csrf_token = CsrfToken::new_random();
        let oauth_user = get_oauth_token(
            client,
            code,
            PkceCodeVerifier::new(user.get_pkce_verifier()),
            http_client,
        )
        .await?;
        create_user(session, uuid, &new_csrf_token, oauth_user).await?;
        Ok(new_csrf_token)
    } else {
        Err("failed to complete oauth2".into())
    }
}

pub async fn delete_oauth(
    session: Session,
    client: &GoogleOAuthClient,
    http_client: &Client,
    uuid: Uuid,
    csrf_token: CsrfToken,
) -> Result<(), Box<dyn Error>> {
    if validate_user::<User>(session.clone(), uuid, csrf_token).await {
        let user = get_user::<User>(session.clone(), uuid).await?;
        let user = user.ok_or("failed to delete session")?;
        let access_token = AccessToken::new(user.access_token().into());
        revoke_token(client, http_client, access_token).await?;
        delete_user(session, uuid).await;
        Ok(())
    } else {
        Err("failed to delete session".into())
    }
}
