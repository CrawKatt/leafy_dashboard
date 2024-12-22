use std::env;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse, TokenUrl, Scope as OAuth2Scope};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: Option<String>,
}

#[get("/api/auth")]
pub async fn auth_redirect() -> impl Responder {
    let client_id = env::var("CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).expect("Invalid auth URL"),
        None,
    )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3000/api/callback".to_string()).expect("Invalid redirect URI"),
        );

    let (auth_url, _csrf_token) = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(OAuth2Scope::new("identify".to_string()))
        .add_scope(OAuth2Scope::new("guilds".to_string()))
        .add_scope(OAuth2Scope::new("guilds.members.read".to_string()))
        .add_scope(OAuth2Scope::new("role_connections.write".to_string()))
        .url();

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/api/callback")]
pub async fn auth_callback(query: web::Query<AuthQuery>) -> impl Responder {
    let client_id = env::var("CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).expect("Invalid auth URL"),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).expect("Invalid token URL")),
    )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3000/api/callback".to_string()).expect("Invalid redirect URI"),
        );

    let client = client
        .exchange_code(AuthorizationCode::new(query.code.clone().unwrap_or_default()))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    let Ok(token) = client else {
        return HttpResponse::InternalServerError().body("Error al obtener el token de acceso")
    };

    let access_token = token
        .access_token()
        .secret();

    let token_cookie = Cookie::build("access_token", access_token)
        .path("/")
        .http_only(true)
        .secure(false)
        .finish();

    HttpResponse::Found()
        .append_header(("Location", "/dashboard"))
        .cookie(token_cookie)
        .finish()
}