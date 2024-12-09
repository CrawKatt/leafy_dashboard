use leptos::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reqwasm::http::Request;
use crate::frontend::components::server_card::{Server, ServerCard};
use crate::models::user::Guild;

#[component]
pub fn Dashboard() -> impl IntoView {
    let (servers, set_servers) = signal(Vec::<Guild>::new());
    let fetch_servers = move || {
        spawn_local(async move {
            let response = Request::get("/api/servers")
                .send()
                .await;

            if let Ok(res) = response {
                if res.status() == 200 {
                    if let Ok(data) = res.json::<Vec<Guild>>().await {
                        set_servers.set(data);
                    }
                }
            }
        });
    };

    fetch_servers();
    /*
    let servers = create_resource(|| async {
        let response = Request::get("/api/servers")
            .send()
            .await
            .unwrap();
        response.json::<Vec<Server>>().await.unwrap()
    });
    */

    view! {
        <div class="min-h-screen bg-gradient-to-b from-green-900 via-green-800 to-green-900">
            <div class="mx-auto max-w-6xl p-6">
                <div class="mb-8 text-center">
                    <h1 class="mb-4 text-4xl font-bold text-green-50">"Select a server"</h1>
                </div>

                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                    {move || {
                        servers.get().iter().map(|guild| {
                            view! {
                                <ServerCard server=Server {
                                    id: guild.id.clone(),
                                    name: guild.name.clone(),
                                    owner: "Owner".to_string(),
                                    icon: Some(guild.icon.clone().unwrap_or("/default-icon.png".to_string()))
                                } />
                            }
                        }).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </div>
    }
}
