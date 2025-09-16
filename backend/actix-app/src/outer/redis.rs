use actix_session::{self, Session, SessionGetError, SessionInsertError};
use chrono::Duration;
use oauth2::{CsrfToken, PkceCodeVerifier};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Debug};
use uuid::Uuid;

use crate::outer::googleoauth::OauthUser;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OAuthSession {
    csrf_token: String,
    pkce_verifier: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    access_token: String,
    //refresh_token: Option<RefreshToken>,
    token_type: String,
    expires_in: Option<Duration>,
    csrf_token: String,
}

pub trait HasCsrf {
    fn csrf_secret(self) -> String;
}

impl HasCsrf for OAuthSession {
    fn csrf_secret(self) -> String {
        self.csrf_token
    }
}

impl OAuthSession {
    pub fn get_pkce_verifier(self) -> String {
        self.pkce_verifier
    }
}

impl HasCsrf for User {
    fn csrf_secret(self) -> String {
        self.csrf_token
    }
}

impl User {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn token_type(&self) -> &str {
        &self.token_type
    }
}

pub async fn create_oauth_session(
    session: Session,
    csrf_token: CsrfToken,
    pkce_verifier: PkceCodeVerifier,
) -> Result<Uuid, SessionInsertError> {
    let uuid = Uuid::new_v4();
    let csrf_token = csrf_token.into_secret();
    let pkce_verifier = pkce_verifier.into_secret();
    let oauthsession = OAuthSession {
        csrf_token,
        pkce_verifier,
    };
    session.insert(&uuid.to_string(), oauthsession)?;
    Ok(uuid)
}

pub async fn get_user<T: HasCsrf + serde::de::DeserializeOwned + Debug>(
    session: Session,
    uuid: Uuid,
) -> Result<Option<T>, SessionGetError> {
    session.get(&uuid.to_string())
}

pub async fn create_user(
    session: Session,
    uuid: Uuid,
    new_csrf_token: &CsrfToken,
    oauth_user: OauthUser,
) -> Result<(), Box<dyn Error>> {
    let uuid = &uuid.to_string();
    let oauth_session: Option<OAuthSession> = session.get(uuid)?;
    match oauth_session {
        Some(_) => {
            session.remove(uuid);
            let csrf_token = new_csrf_token.clone().into_secret();
            let user = User {
                csrf_token,
                access_token: oauth_user.access_token,
                //refresh_token: oauth_user.refresh_token,
                token_type: oauth_user.token_type.as_ref().to_owned(),
                expires_in: oauth_user.expires_in,
            };
            session.insert(uuid, user)?;
            Ok(())
        }
        None => Err("OAuth session not found".into()),
    }
}

pub async fn validate_user<T: HasCsrf + serde::de::DeserializeOwned + Clone + Debug>(
    session: Session,
    uuid: Uuid,
    csrf_token: CsrfToken,
) -> bool {
    if let Ok(Some(has_csrf)) = get_user::<T>(session, uuid).await {
        has_csrf.csrf_secret() == csrf_token.into_secret()
    } else {
        false
    }
}

pub async fn delete_user(session: Session, uuid: Uuid) {
    session.remove(&uuid.to_string());
}
