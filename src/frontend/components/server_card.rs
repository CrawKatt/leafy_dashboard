use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordServer {
    pub guild_id: String,
    pub name: String,
    pub owner: String,
    pub icon: Option<String>,
}

#[component]
pub fn ServerCard(server: DiscordServer) -> impl IntoView {
    let icon_url = server.icon.clone().unwrap_or_else(|| "/default-icon.png".to_string());

    view! {
        <div
            data-key={server.guild_id.clone()}
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
                    >
                        "Go"
                    </button>
                </div>
            </div>
        </div>
    }
}