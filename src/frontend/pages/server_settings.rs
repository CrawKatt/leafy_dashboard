use crate::frontend::components::channel_dropdown::ChannelDropdown;
use crate::frontend::components::header::Header;
use crate::frontend::components::role_dropdown::RoleDropdown;
use crate::frontend::components::sidebar::Sidebar;
use crate::frontend::components::text_card::TextCard;
use crate::frontend::components::user_dropdown::UserDropdown;
use crate::frontend::pages::loading_indicator::LoadingIndicator;
use crate::models::guild::{DiscordChannel, DiscordRole};

use crate::frontend::global_state::GlobalState;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::frontend::components::timeout_dropdown::TimeoutDropdown;
use crate::utils::base_url::BaseUrl;

#[derive(Params, PartialEq)]
pub struct DashboardParams {
    pub guild_id: Option<String>
}

#[component]
pub fn ServerSettings() -> impl IntoView {
    let global_state: GlobalState = use_context().unwrap();
    let active_dropdown = RwSignal::new(None);
    let params = use_params::<DashboardParams>();
    let guild_id = move || params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.guild_id.clone())
        .unwrap_or_default();

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
                let timeout_duration = vec![
                    "1 Minuto".to_string(),
                    "5 Minutos".to_string(),
                    "10 Minutos".to_string(),
                    "1 Hora".to_string(),
                    "1 Día".to_string(),
                    "1 Semana".to_string(),
                ];
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
                                    on_change=Callback::new(move |roles| {
                                        global_state.admin_roles.set(roles)
                                    })
                                />
                                <RoleDropdown
                                    title="Forbidden Roles"
                                    index=1
                                    allow_multiple=true
                                    roles=roles
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |roles: Vec<String>| {
                                        if let Some(role) = roles.first() {
                                            global_state.forbidden_role.set(role.clone());
                                        }
                                    })
                                />
                                <TimeoutDropdown
                                    title="Timeout Duration"
                                    index=2
                                    allow_multiple=false
                                    duration=timeout_duration
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |durations: Vec<String>| {
                                        if let Some(duration_in_seconds) = durations.first() {
                                            global_state.timeout_time.set(duration_in_seconds.clone());
                                        }
                                    })
                                />
                                <ChannelDropdown
                                    title="Welcome Channel"
                                    index=3
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |channels: Vec<String>| {
                                        if let Some(channel) = channels.first() {
                                            global_state.welcome_channel.set(channel.clone());
                                        }
                                    })
                                />
                                <ChannelDropdown
                                    title="Logs Channel"
                                    index=4
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |channels: Vec<String>| {
                                        if let Some(channel) = channels.first() {
                                            global_state.logs_channel.set(channel.clone());
                                        }
                                    })
                                />
                                <ChannelDropdown
                                    title="Exceptions Channel"
                                    index=5
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |channels: Vec<String>| {
                                        if let Some(channel) = channels.first() {
                                            global_state.exceptions_channel.set(channel.clone());
                                        }
                                    })
                                />
                                <ChannelDropdown
                                    title="OOC Channel"
                                    index=6
                                    allow_multiple=false
                                    channels=channels.clone()
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |channels: Vec<String>| {
                                        if let Some(channel) = channels.first() {
                                            global_state.ooc_channel.set(channel.clone());
                                        }
                                    })
                                />
                                <UserDropdown
                                    title="Forbidden User"
                                    index=7
                                    guild_id=guild_id()
                                    active_dropdown=active_dropdown
                                    on_change=Callback::new(move |users: Vec<String>| {
                                        if let Some(user) = users.first() {
                                            global_state.forbidden_user.set(user.clone());
                                        }
                                    })
                                />
                            </div>
                            <div class="grid grid-cols-2 gap-6 p-6">
                                <TextCard
                                    title="Warn Message"
                                    placeholder="Tu mensaje aquí"
                                    on_change=global_state.warn_message
                                />
                                <TextCard
                                    title="Timeout Message"
                                    placeholder="Tu mensaje aquí"
                                    on_change=global_state.timeout_message
                                />
                                <TextCard
                                    title="Welcome Message"
                                    placeholder="Tu mensaje aquí"
                                    on_change=global_state.welcome_message
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
    fetch_and_parse::<Vec<DiscordRole>>(&Client::new(), &format!("{}/api/roles/{guild_id}", BaseUrl::get())).await.unwrap_or_default()
}

async fn fetch_channels(guild_id: String) -> Vec<DiscordChannel> {
    fetch_and_parse::<Vec<DiscordChannel>>(&Client::new(), &format!("{}/api/channels/{guild_id}", BaseUrl::get())).await.unwrap_or_default()
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