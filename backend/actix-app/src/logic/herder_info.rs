use actix_web::{error, Error, HttpRequest};
use oauth2::CsrfToken;
use uuid::Uuid;

pub fn get_header_cookie(req: &HttpRequest) -> Result<(Uuid, CsrfToken), Error> {
    let uuid_cookie = req.cookie("X-Session-Id").ok_or(error::ErrorBadRequest("Missing session cookie"))?;
    let csrf_cookie = req.cookie("X-Csrf-Token").ok_or(error::ErrorBadRequest("Missing csrf cookie"))?;
    let uuid = Uuid::parse_str(uuid_cookie.value()).map_err(|_| error::ErrorBadRequest("Invalid session-id"))?;
    let csrf_token = CsrfToken::new(csrf_cookie.value().to_string());
    Ok((uuid, csrf_token))
}

