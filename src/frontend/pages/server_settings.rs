use std::fmt::Debug;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::components::header::Header;
use crate::frontend::components::sidebar::Sidebar;
use crate::frontend::components::role_dropdown::RoleDropdown;
use crate::frontend::components::server_card::{DiscordChannel, DiscordRole};

use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::frontend::components::channel_dropdown::ChannelDropdown;

#[derive(Params, PartialEq)]
struct DashboardParams {
    guild_id: Option<String>
}

#[component]
pub fn ServerSettings() -> impl IntoView {
    let active_dropdown = RwSignal::new(None);
    let params = use_params::<DashboardParams>();
    let guild_id = move || params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.guild_id.clone())
        .unwrap_or_default();

    let (roles, set_roles) = signal(Vec::<DiscordRole>::new());
    let (channels, set_channels) = signal(Vec::<DiscordChannel>::new());
    spawn_local(async move {
        let client = Client::new();

        let roles_url = format!("http://localhost:3000/api/roles/{}", guild_id());
        let channels_url = format!("http://localhost:3000/api/channels/{}", guild_id());

        let Ok(role_data) = fetch_and_parse::<Vec<DiscordRole>>(&client, &roles_url).await else {
            return log!("Ocurrió un error al realizar la solicitud a la API para obtener los roles")
        };

        let Ok(channel_data) = fetch_and_parse::<Vec<DiscordChannel>>(&client, &channels_url).await else {
            return log!("Ocurrió un error al realizar la solicitud a la API para obtener los canales")
        };

        set_roles.set(role_data);
        set_channels.set(channel_data)
    });

    view! {
        <div class="flex min-h-screen bg-gray-900 text-white">
            <Sidebar />
            <div class="flex-1 flex flex-col">
                <Header title="Bot Configuration" />
                <div class="p-6 grid grid-cols-2 gap-6">
                    <RoleDropdown
                        title="Admin Roles"
                        index={0}
                        allow_multiple=true
                        roles=roles
                        active_dropdown=active_dropdown
                    />
                    <RoleDropdown
                        title="Forbidden Roles"
                        index={1}
                        allow_multiple=true
                        roles=roles
                        active_dropdown=active_dropdown
                    />
                    <ChannelDropdown
                        title="Timeout Duration"
                        index={2}
                        allow_multiple=false
                        channels=channels
                        active_dropdown=active_dropdown
                    />
                    <Card title="Welcome Channel">
                        <Dropdown
                            options={vec!["Channel 1".to_string(), "Channel 2".to_string()]}
                            index={3}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="Logs Channel">
                        <Dropdown
                            options={vec!["Channel 1".to_string(), "Channel 2".to_string()]}
                            index={4}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="Exceptions Channel">
                        <Dropdown
                            options={vec!["Channel 1".to_string(), "Channel 2".to_string()]}
                            index={5}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="OOC Channel">
                        <Dropdown
                            options={vec!["Channel 1".to_string(), "Channel 2".to_string()]}
                            index={6}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                </div>
            </div>
        </div>
    }
}

async fn fetch_and_parse<T: DeserializeOwned + Debug>(
    client: &Client,
    url: &str,
) -> Result<T, String> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Error al conectar con la API: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Error en la respuesta: {}", response.status()));
    }

    response
        .json::<T>()
        .await
        .map_err(|e| format!("Error al deserializar el JSON: {}", e))
}