use actix_session::Session;

use reqwest::Client;
use serde::Deserialize;

use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, error,
    web::{Data, Query},
};

use crate::logic::{get_data::get_youtube_movie, herder_info::get_header_cookie};

#[derive(Deserialize)]
pub struct FormData {
    csrf_token: String,
    list_id: String,
}

#[derive(Deserialize)]
pub struct Code {
    playlist_id: Option<String>,
}

pub async fn get_youtube_playlist(
    session: Session,
    http_client: Data<Client>,
    req: HttpRequest,
    playlist_id: Query<Code>,
) -> Result<impl Responder, Error> {
    let (uuid, _) = get_header_cookie(&req)?;
    let playlist_id: String = playlist_id.into_inner().playlist_id.ok_or(error::ErrorBadRequest("need list id"))?;
    let playlist = get_youtube_movie(
        session,
        http_client.as_ref(),
        uuid,
        &playlist_id,
    )
    .await;
    match playlist {
        Ok(x) => Ok(HttpResponse::Ok().json(x)),
        Err(x) => Err(error::ErrorUnauthorized(x)),
    }
}
