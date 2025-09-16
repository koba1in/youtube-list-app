use crate::outer::{
    redis::{User, get_user},
    youtube::{Playlist, Snippet, get_playlist},
};
use actix_session::Session;
use reqwest::Client;
use std::error::Error;
use uuid::Uuid;

pub async fn get_youtube_movie(
    session: Session,
    http_client: &Client,
    uuid: Uuid,
    playlist_id: &str,
) -> Result<Vec<Snippet>, Box<dyn Error>> {
    if let Ok(Some(user)) = get_user::<User>(session, uuid).await {
        let mut page_token = None;
        let mut list: Vec<Snippet> = Vec::with_capacity(5000);
        loop {
            let playlist = get_playlist(
                http_client,
                user.token_type(),
                user.access_token(),
                playlist_id,
                page_token,
            )
            .await?;
            let Playlist {
                next_page_token,
                items,
            } = playlist;

            page_token = next_page_token;
            let snippets: Vec<Snippet> = items.into_iter().map(|item| item.snippet).collect();
            list.append(&mut snippets.into());

            if page_token.is_none() {
                break;
            }
        }
        return Ok(list);
    }
    Err("authorization is needed".into())
}
