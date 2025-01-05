use leptos::prelude::*;
use crate::models::guild::DiscordChannel;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;

#[component]
pub fn ChannelDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    channels: Vec<DiscordChannel>,
    active_dropdown: RwSignal<Option<usize>>,
    on_change: Callback<Vec<String>>
) -> impl IntoView {
    let channel = channels.iter().map(|channel| channel.name.clone()).collect::<Vec<String>>();
    view! {
        <Card title=title>
            {
                view! {
                    <Dropdown
                        options=channel
                        index=index
                        active_dropdown=active_dropdown
                        allow_multiple=allow_multiple
                        on_change=Callback::new(move |selected: Vec<String>| {
                            if let Some(channel) = selected.first().cloned() {
                                on_change.run(vec![channel]);
                            }
                        })
                    />
                }
            }
        </Card>
    }
}