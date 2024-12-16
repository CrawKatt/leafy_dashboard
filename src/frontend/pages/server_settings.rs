use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reqwest::Client;
use crate::frontend::components::server_card::DiscordServer;

#[derive(Params, PartialEq)]
struct DashboardParams {
    guild_id: Option<String>
}

#[component]
pub fn ServerSettings() -> impl IntoView {
    let params = use_params::<DashboardParams>();
    let guild_id = move || params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.guild_id.clone())
        .unwrap_or_default();

    let (server_data, set_server_data) = signal(None::<DiscordServer>);

    spawn_local(async move {
        let client = Client::new()
            .get(&format!("http://localhost:3000/api/servers/{}", guild_id()))
            .send()
            .await;

        let Ok(response) = client else {
            return log::error!("Ocurrió un error al conectar con la API");
        };

        if response.status().is_success() {
            if let Ok(data) = response.json::<DiscordServer>().await {
                set_server_data.set(Some(data));
            }
        } else {
            log::error!("Falló al consultar servidor: {}", response.status());
        }
    });

    view! {
        <div>
            <h1>"Configuración del Servidor"</h1>
            <Show
                when=move || server_data.get().is_some()
                fallback=move || view! { <p>"Cargando datos del servidor..."</p> }
            >
                {move || {
                    server_data.get().map(|server| {
                        view! {
                            <div>
                                <h2>{format!("Configurando: {}", server.name)}</h2>
                                <img src={server.icon.clone().unwrap_or_else(|| "/default-icon.png".to_string())} />
                            </div>
                        }
                    })
                }}
            </Show>
        </div>
    }
}
