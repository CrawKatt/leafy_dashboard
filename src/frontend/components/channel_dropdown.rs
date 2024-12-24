use leptos::prelude::*;
use crate::models::guild::DiscordChannel;
use crate::frontend::components::card::Card;
use crate::frontend::components::dropdown::Dropdown;

#[component]
pub fn ChannelDropdown(
    title: &'static str,
    index: usize,
    allow_multiple: bool,
    channels: ReadSignal<Vec<DiscordChannel>>,
    active_dropdown: RwSignal<Option<usize>>
) -> impl IntoView {
    view! {
        <Card title=title>
            {move || channels.with(|c| {
                view! {
                    <Dropdown
                        options={c.iter().map(|channel| channel.name.clone()).collect::<Vec<String>>()}
                        index={index}
                        active_dropdown={active_dropdown}
                        allow_multiple={allow_multiple}
                    />
                }
            })}
        </Card>
    }
}