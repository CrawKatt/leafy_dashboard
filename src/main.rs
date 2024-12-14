// Añadir los mod tanto en main.rs como en lib.rs
mod app;
mod frontend;
mod services;
mod models;
mod api; // NO IMPORTAR LOS MÓDULOS DEL BACKEND EN lib.rs

use crate::app::*;
use actix_files::Files;
use actix_web::*;
use dotenv::dotenv;
use leptos::prelude::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope as OAuth2Scope, TokenResponse, TokenUrl};
use std::env;
use actix_web::cookie::Cookie;
use serde::Deserialize;
use crate::api::user::get_servers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .service(auth_redirect)
            .service(get_servers)
            .service(auth_callback)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;

                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }})
            .service(Files::new("/", site_root.as_ref()))
            .wrap(middleware::Compress::default())
    })
        .bind(&addr)?
        .run()
        .await
}

#[get("/api/auth")]
pub async fn auth_redirect() -> impl Responder {
    let client_id = env::var("CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).expect("Invalid auth URL"),
        None, // No especificamos el TokenUrl aquí
    )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3000/api/callback".to_string()).expect("Invalid redirect URI"),
        );

    let (auth_url, _csrf_token) = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(OAuth2Scope::new("identify".to_string()))
        .add_scope(OAuth2Scope::new("guilds".to_string()))
        .url();

    // Redirige al usuario a la página de autorización de Discord
    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

// Estructura para manejar los parámetros de la URL
#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: Option<String>,
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

    // Intercambia el código de autorización por un token de acceso
    match client
        .exchange_code(AuthorizationCode::new(query.code.clone().unwrap()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
    {
        Ok(token) => {
            // Obtén el token de acceso
            let access_token = token.access_token().secret();

            // Configura la cookie con el token de acceso
            let token_cookie = Cookie::build("access_token", access_token.clone())
                .path("/")
                .http_only(true)
                .secure(false) // Cambiar a `true` en producción con HTTPS
                .finish();

            HttpResponse::Found()
                .append_header(("Location", "/dashboard")) // Redirige al Dashboard
                .cookie(token_cookie) // Adjunta la cookie con el token
                .finish()
        }
        Err(err) => {
            println!("Error al intercambiar el código por un token: {:?}", err);
            HttpResponse::InternalServerError().body("Error al obtener el token de acceso")
        }
    }
}