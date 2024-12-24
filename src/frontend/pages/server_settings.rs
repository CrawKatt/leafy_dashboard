use crate::frontend::components::header::Header;
use crate::frontend::components::role_dropdown::RoleDropdown;
use crate::frontend::components::sidebar::Sidebar;
use crate::frontend::components::channel_dropdown::ChannelDropdown;
use crate::models::guild::{DiscordChannel, DiscordRole};

use std::fmt::Debug;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::frontend::components::text_card::TextCard;

#[derive(Params, PartialEq)]
struct DashboardParams {
    guild_id: Option<String>
}

#[component]
pub fn ServerSettings() -> impl IntoView {
    let active_dropdown = RwSignal::new(None);
    let params = use_params::<DashboardParams>();
    let guild_id = params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.guild_id.clone())
        .unwrap_or_default();

    let (roles, set_roles) = signal(Vec::<DiscordRole>::new());
    let (channels, set_channels) = signal(Vec::<DiscordChannel>::new());
    let (message, set_message) = signal(String::new());
    spawn_local(async move {
        if let Err(why) = fetch_and_set_data(&guild_id, set_roles, set_channels).await {
            log!("Error fetching data: {why:#?}")
        }
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
                    <ChannelDropdown
                        title="Welcome Channel"
                        index={3}
                        allow_multiple=false
                        channels=channels
                        active_dropdown=active_dropdown
                    />
                    <ChannelDropdown
                        title="Logs Channel"
                        index={4}
                        allow_multiple=false
                        channels=channels
                        active_dropdown=active_dropdown
                    />
                    <ChannelDropdown
                        title="Exceptions Channel"
                        index={5}
                        allow_multiple=false
                        channels=channels
                        active_dropdown=active_dropdown
                    />
                    <ChannelDropdown
                        title="OOC Channel"
                        index={6}
                        allow_multiple=false
                        channels=channels
                        active_dropdown=active_dropdown
                    />
                    <ChannelDropdown
                        title="Forbidden User"
                        index={7}
                        allow_multiple=false
                        channels=channels
                        active_dropdown=active_dropdown
                    />
                </div>
                <div class="p-6 grid grid-cols-2 gap-6">
                    <TextCard
                        title="Warn Message"
                        placeholder="Tu mensaje aquí"
                        on_change=set_message
                    />
                    <TextCard
                        title="Timeout Message"
                        placeholder="Tu mensaje aquí"
                        on_change=set_message
                    />
                    <TextCard
                        title="Welcome Message"
                        placeholder="Tu mensaje aquí"
                        on_change=set_message
                    />
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
        .map_err(|why| format!("Error al conectar con la API: {why}"))?;

    if !response.status().is_success() {
        return Err(format!("Error en la respuesta: {}", response.status()));
    }

    response
        .json::<T>()
        .await
        .map_err(|why| format!("Error al deserializar el JSON: {why}"))
}

async fn fetch_and_set_data(
    guild_id: &str,
    set_roles: WriteSignal<Vec<DiscordRole>>,
    set_channels: WriteSignal<Vec<DiscordChannel>>
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let roles_url = format!("http://localhost:3000/api/roles/{guild_id}");
    let channels_url = format!("http://localhost:3000/api/channels/{guild_id}");

    let roles = fetch_and_parse::<Vec<DiscordRole>>(&client, &roles_url).await?;
    let channels = fetch_and_parse::<Vec<DiscordChannel>>(&client, &channels_url).await?;

    set_roles.set(roles);
    set_channels.set(channels);

    Ok(())
}