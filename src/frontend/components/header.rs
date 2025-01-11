use crate::frontend::components::save_changes_button::SaveChangesButton;
use crate::frontend::pages::server_settings::DashboardParams;
use leptos::prelude::*;
use leptos_router::hooks::use_params;

#[component]
pub fn Header(title: &'static str) -> impl IntoView {
    let params = use_params::<DashboardParams>();
    let guild_id = move || params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.guild_id.clone())
        .unwrap_or_default();

    view! {
        <div class="flex justify-between items-center p-6 bg-gray-800">
            <h1 class="text-xl font-bold">{title}</h1>
            <SaveChangesButton guild_id=guild_id() />
        </div>
    }
}