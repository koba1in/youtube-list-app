use actix_cors::Cors;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::RedisSessionStore};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    cookie::{Key, time::Duration},
    http::header,
    middleware, web,
};

use reqwest::{Client, ClientBuilder, redirect::Policy};
use serde::{Deserialize, Serialize};
use std::env;
mod handler;
mod logic;
mod outer;

use outer::googleoauth::build_oauth;

use crate::handler::{
    auth::{callback, login, logout},
    data::get_youtube_playlist,
};

use dotenv::dotenv;

fn create_session_middleware(
    store: RedisSessionStore,
    key: Key,
) -> SessionMiddleware<RedisSessionStore> {
    SessionMiddleware::builder(store, key)
        .cookie_name("app_session".into())
        .cookie_secure(true)
        .cookie_content_security(actix_session::config::CookieContentSecurity::Private)
        .cookie_same_site(actix_web::cookie::SameSite::Lax)
        .cookie_path("/".into())
        .cookie_http_only(true)
        .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(1)))
        .build()
}

fn build_http_client() -> Result<Client, reqwest::Error> {
    Ok(ClientBuilder::new().redirect(Policy::none()).build()?)
}

#[derive(Serialize)]
struct NotFoundJson {
    error: String,
}

async fn not_found() -> HttpResponse {
    let error = "Not Found".to_string();
    HttpResponse::NotFound().json(NotFoundJson { error })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let store: RedisSessionStore = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to connect redis"))?;

    let key = Key::generate();

    let http_client =
        web::Data::new(build_http_client().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "failed to build reqwest")
        })?);
    let oauth_client = web::Data::new(build_oauth().await.map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::Other, "failed to build oauth client")
    })?);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://localhost:5000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
                    .supports_credentials(),
            )
            .wrap(middleware::Compress::default())
            .wrap(create_session_middleware(store.clone(), key.clone()))
            .app_data(http_client.clone())
            .app_data(oauth_client.clone())
            .route("/auth/login", web::get().to(login))
            .route("/auth/callback", web::get().to(callback))
            .route("/auth/logout", web::post().to(logout))
            .route("/data", web::get().to(get_youtube_playlist))
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
