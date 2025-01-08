use leptos::prelude::*;
use crate::models::guild::DiscordChannel;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;
use crate::frontend::utils::generate_ids;

#[component]
pub fn ChannelDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    channels: Vec<DiscordChannel>,
    active_dropdown: RwSignal<Option<usize>>,
    on_change: Callback<Vec<String>>,
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || {
                let options = channels
                    .iter()
                    .map(|channel| (channel.id.clone(), channel.name.clone()))
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