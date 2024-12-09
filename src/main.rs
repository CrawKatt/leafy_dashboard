mod app;

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
    let auth_url = env::var("AUTH_URL").expect("DISCORD_AUTH_URL not set");
    let redirect_uri = env::var("CALLBACK_URL").expect("DISCORD_REDIRECT_URI not set");
    let client_secret = env::var("CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).expect("Invalid auth URL"),
        None
    )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URI"));

    let auth_request = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(OAuth2Scope::new("identify".to_string()));

    let auth_url = auth_request.url().0.to_string();

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

/*
#[get("/api/callback")]
async fn callback(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let code = match query.get("code") {
        Some(code) => code.clone(),
        None => return HttpResponse::BadRequest().body("Authorization code is missing."),
    };

    let db = setup_db().await;
    let client_id = env::var("CLIENT_ID").expect("DISCORD_CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET not set");
    let token_url = env::var("DISCORD_TOKEN_URL").expect("DISCORD_TOKEN_URL not set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(env::var("AUTH_URL").expect("Invalid auth URL")).expect("Invalid auth URL"),
        Some(TokenUrl::new(token_url).expect("Invalid token URL")),
    )
        .set_redirect_uri(
            RedirectUrl::new(env::var("CALLBACK_URL").expect("Invalid redirect URI"))
                .expect("Invalid redirect URI"),
        );

    // Intercambiar el código por un token
    let token_response = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await;

    let token = match token_response {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().body("Error during token exchange."),
    };

    let access_token = token.access_token().secret().to_string();

    HttpResponse::Ok()
    //let user_info = get_user_info(&access_token).await;

    //resolve_user_info(db, token, user_info).await
}
*/
