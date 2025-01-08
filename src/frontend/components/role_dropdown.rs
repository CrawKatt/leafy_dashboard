use leptos::prelude::*;
use leptos::prelude::RwSignal;
use crate::models::guild::DiscordRole;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::utils::generate_ids;

#[component]
pub fn RoleDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    roles: Vec<DiscordRole>,
    active_dropdown: RwSignal<Option<usize>>,
    on_change: Callback<Vec<String>>, // Ahora esto será un vector de IDs
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || {
                let options = roles
                    .iter()
                    .map(|role| (role.id.clone(), role.name.clone()))
                    .collect::<Vec<(String, String)>>();
                view! {
                    <Dropdown
                        options={options
                            .iter()
                            .map(|(_, name)| name.clone())
                            .collect::<Vec<String>>()}
                        index=index
                        active_dropdown=active_dropdown
                        allow_multiple=allow_multiple
                        on_change=Callback::new(move |selected_names: Vec<String>| {
                            let selected_ids = generate_ids(&options, &selected_names);
                            on_change.run(selected_ids);
                        })
                    />
                }
            }}
        </Card>
    }
}