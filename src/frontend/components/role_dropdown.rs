use leptos::prelude::*;
use leptos::prelude::RwSignal;
use crate::models::guild::DiscordRole;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;

#[component]
pub fn RoleDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    roles: Vec<DiscordRole>,
    active_dropdown: RwSignal<Option<usize>>
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || {
                view! {
                    <Dropdown
                        options={roles
                            .iter()
                            .map(|role| role.name.clone())
                            .collect::<Vec<String>>()}
                        index=index
                        active_dropdown=active_dropdown
                        allow_multiple=allow_multiple
                    />
                }
            }}
        </Card>
    }
}