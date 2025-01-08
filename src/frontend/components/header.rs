use leptos::prelude::*;
use leptos_router::hooks::use_params;
use crate::frontend::components::save_changes_button::SaveChangesButton;
use crate::frontend::global_state::GlobalState;
use crate::frontend::pages::server_settings::DashboardParams;

#[component]
pub fn Header(title: &'static str) -> impl IntoView {
    let global_state: GlobalState = use_context().unwrap();
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
            <SaveChangesButton
                admin_roles=global_state.admin_roles
                guild_id=guild_id()
                forbidden_user=global_state.forbidden_user
                timeout_time=global_state.timeout_time
                forbidden_role=global_state.forbidden_role
                welcome_channel=global_state.welcome_channel
                logs_channel=global_state.logs_channel
                exceptions_channel=global_state.exceptions_channel
                ooc_channel=global_state.ooc_channel
                warn_message=global_state.warn_message
                timeout_message=global_state.timeout_message
                welcome_message=global_state.welcome_message
            />
        </div>
    }
}