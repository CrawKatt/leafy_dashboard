use leptos::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordServer {
    pub guild_id: String,
    pub name: String,
    pub owner: String,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordRole {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordChannel {
    pub id: String,
    pub r#type: u8, // El tipo de canal (0 = texto, 2 = voz, etc.)
    pub guild_id: Option<String>,
    pub name: String,
    pub parent_id: Option<String>,
    pub position: i32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: Option<bool>,
    pub topic: Option<String>, // Solo para canales de texto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    pub id: String,
    pub r#type: u8, // 0 = rol, 1 = usuario
    pub allow: String,
    pub deny: String,
}

#[component]
pub fn ServerCard(server: DiscordServer) -> impl IntoView {
    let icon_url = server.icon.clone().unwrap_or_else(|| "/default-icon.png".to_string());
    let navigate = use_navigate();
    let guild_id = server.guild_id.clone();
    let guild_id_clone = guild_id.clone();

    view! {
        <div
            data-key={guild_id.clone()}
            class="group overflow-hidden border-green-700/30 bg-green-950/30 backdrop-blur-sm transition-colors hover:bg-green-950/50"
        >
            <div class="p-6">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div class="relative h-16 w-16 overflow-hidden rounded-full border-2 border-green-600/30 bg-green-800/30">
                            <img
                                src={icon_url}
                                alt={server.name.clone()}
                                class="object-cover transition-transform group-hover:scale-110"
                            />
                        </div>
                        <div>
                            <h2 class="text-lg font-semibold text-green-50">{server.name.clone()}</h2>
                            <p class="text-sm text-green-300">{server.owner}</p>
                        </div>
                    </div>
                    <button
                        class="border border-green-600/30 bg-green-800/30 text-green-50 hover:bg-green-800/50 hover:text-green-100 px-4 py-2 rounded"
                        on:click=move |_| {
                            navigate(format!("/dashboard/{}", guild_id_clone).as_str(), Default::default());
                        }
                    >
                        "Go"
                    </button>
                </div>
            </div>
        </div>
    }
}