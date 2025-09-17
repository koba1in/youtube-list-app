use actix_cors::Cors;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::RedisSessionStore};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer,
    cookie::{Key, time::Duration},
    http::header,
    middleware, web,
};

use hex::decode;
use reqwest::{Client, ClientBuilder, redirect::Policy};
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{Error, ErrorKind},
};
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
    let redis_url = env::var("REDIS_URL")
        .map_err(|_| Error::new(ErrorKind::Other, "failed to get redis url"))?;
    let store: RedisSessionStore = RedisSessionStore::new(redis_url)
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "failed to connect redis"))?;
    let front_origin = env::var("FRONT_ORIGIN")
        .map_err(|_| Error::new(ErrorKind::Other, "failed to get front origin"))?;
    let key_hex = env::var("KEY").map_err(|_| Error::new(ErrorKind::Other, "failed to get key"))?;
    let key = decode(key_hex).map_err(|_| Error::new(ErrorKind::Other, "failed to get key"))?;
    let key = Key::from(key.as_ref());

    let http_client =
        web::Data::new(build_http_client().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "failed to build reqwest")
        })?);
    let oauth_client = web::Data::new(build_oauth().await.map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::Other, "failed to build oauth client")
    })?);
    let front_origin_data = web::Data::new(front_origin.clone());
    HttpServer::new(move || {
        App::new()
            .wrap(create_session_middleware(store.clone(), key.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin(&front_origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
                    .supports_credentials(),
            )
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::default()
                    .add((
                        "Strict-Transport-Security",
                        "max-age=31536000; includeSubDomains",
                    ))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add((
                        "Content-Security-Policy",
                        "script-src 'nonce-random-nonce-value",
                    ))
                    .add(("Referrer-Policy", "no-referrer")),
            )
            .app_data(http_client.clone())
            .app_data(oauth_client.clone())
            .app_data(front_origin_data.clone())
            .route("/auth/login", web::get().to(login))
            .route("/auth/callback", web::get().to(callback))
            .route("/auth/logout", web::post().to(logout))
            .route("/data", web::get().to(get_youtube_playlist))
            .default_service(web::to(not_found))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
