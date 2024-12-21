use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::components::header::Header;
use crate::frontend::components::sidebar::Sidebar;

use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct DashboardParams {
    guild_id: Option<String>
}

#[component]
pub fn ServerSettings() -> impl IntoView {
    let params = use_params::<DashboardParams>();
    let guild_id = move || params
        .read()
        .as_ref()
        .ok()
        .and_then(|params| params.guild_id.clone())
        .unwrap_or_default();

    view! {
        <div class="flex min-h-screen bg-gray-900 text-white">
            <Sidebar />
            <div class="flex-1 flex flex-col">
                <Header title="Bot Configuration" />
                <div class="p-6 grid grid-cols-2 gap-6">
                    <Card title="Admin Roles">
                        <Dropdown options={vec!["Jardinero", "Arquitecto", "Helper", "Mei-chan", "Chikistrikis"]} />
                    </Card>
                    <Card title="Forbidden Roles">
                        <Dropdown options={vec!["Jardinero", "Arquitecto", "Helper", "Mei-chan", "Chikistrikis"]} />
                    </Card>
                    <Card title="Timeout Duration">
                        <Dropdown options={vec!["1 hour", "6 hours", "1 day"]} />
                    </Card>
                    <Card title="Welcome Channel">
                        <Dropdown options={vec!["Channel 1", "Channel 2"]} />
                    </Card>
                    <Card title="Logs Channel">
                        <Dropdown options={vec!["Channel 1", "Channel 2"]} />
                    </Card>
                    <Card title="Exceptions Channel">
                        <Dropdown options={vec!["Channel 1", "Channel 2"]} />
                    </Card>
                </div>
            </div>
        </div>
    }
}