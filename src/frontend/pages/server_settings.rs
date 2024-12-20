use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reqwest::Client;
use crate::frontend::components::role_selector::{Role, RoleSelector};
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
    let (roles, set_roles) = signal(Vec::<Role>::new());
    let (selected_roles, set_selected_roles) = signal(Vec::<String>::new());

    spawn_local(async move {
        let client = Client::new();
        if let Ok(response) = client.get(&format!("http://localhost:3000/api/roles/{}", guild_id()))
            .send()
            .await
        {
            if let Ok(data) = response.json::<Vec<Role>>().await {
                set_roles.set(data);
            }
        }
    });

    view! {
        <div>
            <h1>"Configuración del Servidor"</h1>
            <RoleSelector />
        </div>
    }
}
