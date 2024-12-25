use leptos::logging::log;
use leptos::prelude::*;
use reqwest::Client;
use crate::models::guild::DiscordUser;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::pages::server_settings::fetch_and_parse;

#[component]
pub fn UserDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    guild_id: String,
    active_dropdown: RwSignal<Option<usize>>
) -> impl IntoView {
    // Estado para manejar el término de búsqueda
    let (search_term, set_search_term) = signal(String::new());

    // Crea el recurso local usando el término de búsqueda
    let users_resource = LocalResource::new(move || fetch_users(guild_id.clone(), search_term.get()));

    view! {
        <Card title=title>
            <div class="mb-4">
                <input
                    type="text"
                    class="w-full p-3 border border-gray-600 rounded-lg bg-gray-700 text-gray-200"
                    placeholder="Escribe el nombre de un miembro"
                    on:input=move |e| {
                        let value = event_target_value(&e);
                        set_search_term.set(value); // Actualiza el estado con el término de búsqueda
                    }
                />
            </div>
            <Suspense fallback=move || view! { <p>"Cargando usuarios..."</p> }>
                {move || Suspend::new(async move {
                    let users = users_resource.await;
                    let user_names = users.iter().map(|u| u.user.username.clone()).collect::<Vec<String>>();
                    view! {
                        <Dropdown
                            options={user_names}
                            index={index}
                            active_dropdown={active_dropdown}
                            allow_multiple={allow_multiple}
                        />
                    }
                })}
            </Suspense>
        </Card>
    }
}

async fn fetch_users(guild_id: String, target_user: String) -> Vec<DiscordUser> {
    fetch_and_parse::<Vec<DiscordUser>>(&Client::new(), &format!("http://localhost:3000/api/users/{guild_id}/{target_user}"))
        .await
        .map_err(|why| log!("error al hacer fetch a la API {why}"))
        .unwrap_or_default()
}