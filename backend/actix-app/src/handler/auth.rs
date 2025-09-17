use actix_session::Session;

use actix_web::{
    Error, HttpRequest, HttpResponse, Responder,
    cookie::{Cookie, SameSite},
    error,
    web::{Data, Form, Json, Query},
};
use oauth2::{AuthorizationCode, CsrfToken};
use reqwest::Client;

use crate::logic::{
    herder_info::get_header_cookie,
    oauth::{complete_oauth, start_oauth},
};
use crate::{logic::oauth::delete_oauth, outer::googleoauth::GoogleOAuthClient};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Code {
    code: Option<String>,
    state: Option<String>,
    scope: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CsrfData {
    csrf_token: String,
}

pub async fn login(
    session: Session,
    client: Data<GoogleOAuthClient>,
) -> Result<impl Responder, Error> {
    let (url, uuid, csrf_token) = start_oauth(session, client.get_ref())
        .await
        .map_err(|_| error::ErrorInternalServerError("failed to start OAuth2"))?;
    let uuid = Cookie::build("X-Session-Id", uuid.to_string())
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();
    let csrf_token = Cookie::build("X-Csrf-Token", csrf_token.secret().to_string())
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();
    Ok(HttpResponse::Found()
        .append_header(("Location", url.to_string()))
        .cookie(uuid)
        .cookie(csrf_token)
        .finish())
}

pub async fn callback(
    session: Session,
    client: Data<GoogleOAuthClient>,
    http_client: Data<Client>,
    req: HttpRequest,
    query: Query<Code>,
    front_origin: Data<String>,
) -> Result<impl Responder, Error> {
    let (uuid, csrf_token) = get_header_cookie(&req)?;
    let Code { code, state, scope } = query.into_inner();
    let state = state.ok_or(error::ErrorBadRequest("Missing state"))?;
    if state != csrf_token.secret().to_owned() {
        return Err(error::ErrorUnauthorized("Invalid state/csrf"));
    }
    if let Some(code) = code {
        let code = AuthorizationCode::new(code);
        let new_csrf_token = complete_oauth(
            session,
            client.as_ref(),
            http_client.as_ref(),
            uuid,
            csrf_token,
            code,
        )
        .await
        .map_err(|_| error::ErrorInternalServerError("failed to complete oauth"))?;
        return Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(format!(
                r#"
        <script>
            window.opener.postMessage({{ csrf_token: "{}" }}, "{}");
            window.close();
        </script>
        "#,
                new_csrf_token.into_secret(),
                front_origin.into_inner()
            )));
    }
    Err(error::ErrorInternalServerError(
        "An unexpected error occurred. Please try again later.",
    ))
}

pub async fn logout(
    session: Session,
    client: Data<GoogleOAuthClient>,
    http_client: Data<Client>,
    req: HttpRequest,
    form: Json<CsrfData>,
) -> Result<impl Responder, Error> {
    let (uuid, _) = get_header_cookie(&req)?;
    let csrf_token = CsrfToken::new(form.into_inner().csrf_token);
    match delete_oauth(
        session,
        client.get_ref(),
        http_client.get_ref(),
        uuid,
        csrf_token,
    )
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json("Successfully logged out")),
        Err(_) => Err(error::ErrorInternalServerError(
            "An unexpected error occurred. Please try again later.",
        )),
    }
}
