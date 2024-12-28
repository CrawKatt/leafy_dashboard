use crate::frontend::components::header::Header;
use crate::frontend::components::role_dropdown::RoleDropdown;
use crate::frontend::components::sidebar::Sidebar;
use crate::frontend::components::channel_dropdown::ChannelDropdown;
use crate::frontend::pages::loading_indicator::LoadingIndicator;
use crate::frontend::components::text_card::TextCard;
use crate::frontend::components::user_dropdown::UserDropdown;
use crate::models::guild::{DiscordChannel, DiscordRole};

use std::fmt::Debug;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reqwest::Client;
use serde::de::DeserializeOwned;

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

    let (_message, set_message) = signal(String::new());

    // Para obtener datos desde la API utilizar `LocalResource` y `<Suspense>` dentro del `view!`
    let roles = LocalResource::new(move || fetch_roles(guild_id()));
    let channels = LocalResource::new(move || fetch_channels(guild_id()));

    view! {
        <Suspense fallback=move || {
            view! { <LoadingIndicator /> }
        }>
            {move || Suspend::new(async move {
                let roles = roles.await;
                let channels = channels.await;
                view! {
                    <div class="flex min-h-screen text-white bg-gray-900">
                        <Sidebar />
                        <div class="flex flex-col flex-1">
                            <Header title="Bot Configuration" />
                            <div class="grid grid-cols-2 gap-6 p-6">
                                <RoleDropdown
                                    title="Admin Roles"
                                    index=0
                                    allow_multiple=true
                                    roles=roles.clone()
                                    active_dropdown=active_dropdown
                                />
                                <RoleDropdown
                                    title="Forbidden Roles"
                                    index=1
                                    allow_multiple=true
                                    roles=roles
                                    active_dropdown=active_dropdown
                                />
                                <ChannelDropdown
                                    title="Timeout Duration"
                                    index=2
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                />
                                <ChannelDropdown
                                    title="Welcome Channel"
                                    index=3
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                />
                                <ChannelDropdown
                                    title="Logs Channel"
                                    index=4
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                />
                                <ChannelDropdown
                                    title="Exceptions Channel"
                                    index=5
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                />
                                <ChannelDropdown
                                    title="OOC Channel"
                                    index=6
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                />
                                <UserDropdown
                                    title="Forbidden User"
                                    index=7
                                    guild_id=guild_id()
                                    active_dropdown=active_dropdown
                                />
                            </div>
                            <div class="grid grid-cols-2 gap-6 p-6">
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
            })}
        </Suspense>
    }
}

async fn fetch_roles(guild_id: String) -> Vec<DiscordRole> {
    fetch_and_parse::<Vec<DiscordRole>>(&Client::new(), &format!("http://localhost:3000/api/roles/{guild_id}")).await.unwrap_or_default()
}

async fn fetch_channels(guild_id: String) -> Vec<DiscordChannel> {
    fetch_and_parse::<Vec<DiscordChannel>>(&Client::new(), &format!("http://localhost:3000/api/channels/{guild_id}")).await.unwrap_or_default()
}

pub async fn fetch_and_parse<T: DeserializeOwned + Debug>(
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