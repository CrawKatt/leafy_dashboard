use leptos::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::frontend::components::server_card::{Server, ServerCard};

// LADO DEL SERVIDOR, NO USAR REQWASM NI COSAS CON WASM. UTILIZAR ÚNICAMENTE CÓDIGO NATIVO
#[component]
pub fn Dashboard() -> impl IntoView {
    let (servers, set_servers) = signal(Vec::<Server>::new());
    let fetch_servers = move || {
        spawn_local(async move {
            match reqwest::Client::new()
                .get("http://localhost:3000/api/servers")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(data) = response.json::<Vec<Server>>().await {
                            set_servers.set(data);
                        }
                    } else {
                        log::error!("Falló al consultar servidores: {}", response.status());
                    }
                }
                Err(e) => {
                    log::error!("Error al conectar con la API: {:?}", e);
                }
            }
        });
    };

    fetch_servers();

    view! {
        <div class="min-h-screen bg-gradient-to-b from-green-900 via-green-800 to-green-900">
            <div class="mx-auto max-w-6xl p-6">
                <div class="mb-8 text-center">
                    <h1 class="mb-4 text-4xl font-bold text-green-50">"Select a server"</h1>
                </div>

                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                    {move || {
                        servers.get().iter().map(|server| {
                            view! {
                                <ServerCard server=server.clone() />
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}