use leptos::prelude::*;
use crate::models::guild::DiscordUser;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;

#[component]
pub fn UserDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    users: ReadSignal<Vec<DiscordUser>>,
    active_dropdown: RwSignal<Option<usize>>
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || users.with(|u| {
                view! {
                    <Dropdown
                        options={u.iter().map(|user| user.name.clone()).collect::<Vec<String>>()}
                        index={index}
                        active_dropdown={active_dropdown}
                        allow_multiple={allow_multiple}
                    />
                }
            })}
        </Card>
    }
}