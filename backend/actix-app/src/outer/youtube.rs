use reqwest::{Client, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub next_page_token: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub title: String,
    pub channel_title: String,
    pub resource_id: ResourceId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceId {
    pub video_id: String,
}

pub async fn get_playlist(
    http_client: &Client,
    token_type: &str,
    access_token: &str,
    playlist_id: &str,
    next_page_token: Option<String>,
) -> Result<Playlist, Box<dyn Error>> {
    let api_key = &env::var("API_KEY")?;

    let res = if let Some(next_page_token) = next_page_token {
        http_client
            .get("https://www.googleapis.com/youtube/v3/playlistItems")
            .header(AUTHORIZATION, [token_type, access_token].join(" "))
            .query(&[
                ("part", "snippet"),
                ("maxResults", "50"),
                ("playlistId", playlist_id),
                ("pageToken", &next_page_token),
                ("key", api_key),
            ])
            .send()
            .await?
    } else {
        http_client
            .get("https://www.googleapis.com/youtube/v3/playlistItems")
            .header(AUTHORIZATION, [token_type, access_token].join(" "))
            .query(&[
                ("part", "snippet"),
                ("maxResults", "50"),
                ("playlistId", playlist_id),
                ("key", api_key),
            ])
            .send()
            .await?
    };
    let json = res.json().await?;
    Ok(json)
}
