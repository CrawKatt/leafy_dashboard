// Añadir los mod tanto en main.rs como en lib.rs
mod app;
mod frontend;
mod services;
mod models;

use crate::app::*;
use actix_files::Files;
use actix_web::*;
use dotenv::dotenv;
use leptos::prelude::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope as OAuth2Scope};
use std::env;
use tailwind_actix::api::user::get_servers;

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
async fn auth_redirect() -> HttpResponse {
    let client_id = env::var("CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).expect("Invalid auth URL"),
        None
    )
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000/dashboard".to_string()).expect("Invalid redirect URI"));

    let auth_request = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(OAuth2Scope::new("identify".to_string()));

    let auth_url = auth_request.url().0.to_string();

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}