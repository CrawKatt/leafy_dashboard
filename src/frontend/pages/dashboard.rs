use leptos::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::models::guild::DiscordServer;
use crate::frontend::components::server_card::ServerCard;
use crate::utils::BASE_URL;

// LADO DEL SERVIDOR, NO USAR REQWASM NI COSAS CON WASM. UTILIZAR ÚNICAMENTE CÓDIGO NATIVO
#[component]
pub fn Dashboard() -> impl IntoView {
    let (servers, set_servers) = signal(Vec::<DiscordServer>::new());
    let fetch_servers = move || {
        spawn_local(async move {
            let client = reqwest::Client::new()
                .get(format!("{BASE_URL}/api/servers"))
                .send()
                .await;

            let Ok(response) = client else {
                return log::error!("Ocurrió un error al conectar con la API")
            };

            if response.status().is_success() {
                if let Ok(data) = response.json::<Vec<DiscordServer>>().await {
                    set_servers.set(data);
                }
            } else {
                log::error!("Falló al consultar servidores: {}", response.status());
            }
        });
    };

    fetch_servers();

    view! {
        <div class="min-h-screen bg-gradient-to-b from-green-900 via-green-800 to-green-900">
            <div class="p-6 mx-auto max-w-6xl">
                <div class="mb-8 text-center">
                    <h1 class="mb-4 text-4xl font-bold text-green-50">"Select a server"</h1>
                </div>

                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                    {move || {
                        servers
                            .get()
                            .iter()
                            .map(|server| {
                                view! { <ServerCard server=server.clone() /> }
                            })
                            .collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}