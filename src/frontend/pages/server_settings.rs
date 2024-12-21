use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::components::header::Header;
use crate::frontend::components::sidebar::Sidebar;

use leptos::prelude::*;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct DashboardParams {
    guild_id: Option<String>
}

#[component]
pub fn ServerSettings() -> impl IntoView {
    let active_dropdown = RwSignal::new(None);

    view! {
        <div class="flex min-h-screen bg-gray-900 text-white">
            <Sidebar />
            <div class="flex-1 flex flex-col">
                <Header title="Bot Configuration" />
                <div class="p-6 grid grid-cols-2 gap-6">
                    <Card title="Admin Roles">
                        <Dropdown
                            options={vec!["Jardinero", "Arquitecto", "Helper", "Mei-chan", "Chikistrikis"]}
                            index={0}
                            active_dropdown={active_dropdown}
                            allow_multiple=true
                        />
                    </Card>
                    <Card title="Forbidden Roles">
                        <Dropdown
                            options={vec!["Jardinero", "Arquitecto", "Helper", "Mei-chan", "Chikistrikis"]}
                            index={1}
                            active_dropdown={active_dropdown}
                            allow_multiple=true
                        />
                    </Card>
                    <Card title="Timeout Duration">
                        <Dropdown
                            options={vec!["1 minute", "1 hour", "1 day", "1 week"]}
                            index={2}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="Welcome Channel">
                        <Dropdown
                            options={vec!["Channel 1", "Channel 2"]}
                            index={3}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="Logs Channel">
                        <Dropdown
                            options={vec!["Channel 1", "Channel 2"]}
                            index={4}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="Exceptions Channel">
                        <Dropdown
                            options={vec!["Channel 1", "Channel 2"]}
                            index={5}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                    <Card title="OOC Channel">
                        <Dropdown
                            options={vec!["Channel 1", "Channel 2"]}
                            index={6}
                            active_dropdown={active_dropdown}
                            allow_multiple=false
                        />
                    </Card>
                </div>
            </div>
        </div>
    }
}