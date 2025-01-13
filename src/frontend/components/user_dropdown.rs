use leptos::logging::log;
use leptos::prelude::*;
use reqwest::Client;
use crate::models::guild::DiscordUser;
use crate::frontend::components::card::Card;
use crate::frontend::pages::server_settings::fetch_and_parse;
use crate::utils::base_url::BaseUrl;

#[component]
pub fn UserDropdown(
    title: &'static str,
    index: usize,
    guild_id: String,
    active_dropdown: RwSignal<Option<usize>>,
    on_change: Callback<Vec<String>>
) -> impl IntoView {
    let (selected_options, set_selected_options) = signal(vec![]);
    let (search_term, set_search_term) = signal(String::new());

    let show_options = move || active_dropdown.get() == Some(index);

    let users = AsyncDerived::new_unsync(move || fetch_users(guild_id.clone(), search_term.get()));

    view! {
        <Card title=title>
            <div
                class="flex justify-between items-center p-3 w-full bg-gray-700 rounded-lg border border-gray-600 cursor-pointer hover:border-indigo-500"
                on:click=move |_| {
                    active_dropdown
                        .update(|active| {
                            if *active == Some(index) {
                                *active = None;
                            } else {
                                *active = Some(index);
                            }
                        });
                }
            >
                <span class="text-gray-200">
                    {move || {
                        if selected_options.get().is_empty() {
                            "Select options...".to_string()
                        } else {
                            selected_options.get().join(", ")
                        }
                    }}
                </span>
                <i class=move || {
                    if show_options() { "icon-chevron-up" } else { "icon-chevron-down" }
                }></i>
            </div>
            <div class="relative">
                <div class=move || {
                    format!(
                        "absolute w-full bg-gray-700 border border-gray-600 rounded-lg shadow-lg z-50 transition-transform duration-200 overflow-hidden {}",
                        if show_options() {
                            "translate-y-0 opacity-100 max-h-60"
                        } else {
                            "-translate-y-4 opacity-0 max-h-0"
                        },
                    )
                }>
                    <div class="p-3">
                        <input
                            type="text"
                            class="py-2 px-3 w-full text-gray-200 bg-gray-600 rounded"
                            placeholder="Search..."
                            on:input=move |ev| {
                                let input = event_target_value(&ev);
                                set_search_term.set(input);
                            }
                        />
                    </div>
                    <div class="overflow-y-auto p-3 h-64">
                        <Suspense fallback=move || {
                            view! { <p class="text-gray-400">"Cargando usuarios..."</p> }
                        }>
                            {move || Suspend::new(async move {
                                let users = users.await;
                                let options = users
                                    .iter()
                                    .map(|user| (user.user.id.clone(), user.user.username.clone()))
                                    .collect::<Vec<(String, String)>>();

                                view! {
                                    <div>
                                        {options
                                            .into_iter()
                                            .map(|(id, name)| {
                                                let name_clone = name.clone();
                                                let is_selected = move || {
                                                    selected_options.get().contains(&name_clone)
                                                };

                                                view! {
                                                    <div
                                                        class="flex justify-between items-center p-3 rounded cursor-pointer hover:bg-gray-600"
                                                        on:click=move |_| {
                                                            set_selected_options
                                                                .update(|current| {
                                                                    if current.contains(&name) {
                                                                        current.clear();
                                                                    } else {
                                                                        current.clear();
                                                                        current.push(name.clone());
                                                                    }
                                                                    on_change.run(vec![id.clone()]);
                                                                });
                                                        }
                                                    >
                                                        <span class="text-gray-200">{name.clone()}</span>
                                                        <i class=move || {
                                                            if is_selected() {
                                                                "icon-check text-indigo-500"
                                                            } else {
                                                                ""
                                                            }
                                                        }></i>
                                                    </div>
                                                }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                }
                            })}
                        </Suspense>
                    </div>
                </div>
            </div>
        </Card>
    }

}

async fn fetch_users(guild_id: String, target_user: String) -> Vec<DiscordUser> {
    if target_user.is_empty() {
        return vec![DiscordUser::default()]
    }

    fetch_and_parse::<Vec<DiscordUser>>(&Client::new(), &format!("{}/api/users/{guild_id}/{target_user}", BaseUrl::get()))
        .await
        .map_err(|why| log!("error al hacer fetch a la API {why}"))
        .unwrap_or_default()
}
