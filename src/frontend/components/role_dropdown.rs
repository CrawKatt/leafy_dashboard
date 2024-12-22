use leptos::prelude::*;
use leptos::prelude::{ReadSignal, RwSignal};
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::components::server_card::DiscordRole;

#[component]
pub fn RoleDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    roles: ReadSignal<Vec<DiscordRole>>,
    active_dropdown: RwSignal<Option<usize>>
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || roles.with(|r| {
                view! {
                    <Dropdown
                        options={r.iter().map(|role| role.name.clone()).collect::<Vec<String>>()}
                        index={index}
                        active_dropdown={active_dropdown}
                        allow_multiple={allow_multiple}
                    />
                }
            })}
        </Card>
    }
}