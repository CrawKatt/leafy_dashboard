// Añadir los mod tanto en main.rs como en lib.rs
mod app;
mod frontend;
mod services;
mod models;
mod api; // NO IMPORTAR LOS MÓDULOS DEL BACKEND EN lib.rs

use crate::api::user::get_users;
use crate::app::*;
use actix_files::Files;
use actix_web::*;
use dotenv::dotenv;
use leptos::prelude::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use leptos_meta::MetaTags;
use crate::api::auth::{auth_callback, auth_redirect};
use crate::api::channel::get_channels;
use crate::api::guild::{get_guild_id, get_servers};
use crate::api::role::get_roles;

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
            .service(get_guild_id)
            .service(get_roles)
            .service(get_channels)
            .service(get_users)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    use leptos::prelude::*;

                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
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