use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::models::guild::{Admin, Channels, Forbidden, GuildData, Messages, TimeOut};

#[component]
pub fn SaveChangesButton(
    admin_roles: RwSignal<Vec<String>>,
    guild_id: String,
    forbidden_user: RwSignal<String>,
    forbidden_role: RwSignal<String>,
    timeout_time: RwSignal<String>,
    welcome_channel: RwSignal<String>,
    ooc_channel: RwSignal<String>,
    logs_channel: RwSignal<String>,
    exceptions_channel: RwSignal<String>,
    welcome_message: RwSignal<String>,
    timeout_message: RwSignal<String>,
    warn_message: RwSignal<String>
) -> impl IntoView {
    let save_settings = move |_| {
        let data_to_save = GuildData {
            admins: Admin {
                role: admin_roles.get()
            },
            guild_id: guild_id.clone(),
            id: None,
            forbidden: Forbidden {
                user: forbidden_user.get(),
                role: forbidden_role.get(),
            },
            time_out: TimeOut {
                time: timeout_time.get()
            },
            channels: Channels {
                welcome: welcome_channel.get(),
                ooc: ooc_channel.get(),
                logs: logs_channel.get(),
                exceptions: exceptions_channel.get(),
            },
            messages: Messages {
                welcome: welcome_message.get(),
                time_out: timeout_message.get(),
                warn: warn_message.get()
            }
        };

        log!("Datos para enviar: {:#?}", data_to_save);

        spawn_local(async move {
            let client = reqwest::Client::new();
            let response = client
                .put("http://localhost:3000/api/save_settings")
                .json(&data_to_save)
                .send()
                .await;

            match response {
                Ok(code) => log!("Settins saved successfully {code:#?}"),
                Err(why) => error!("Failed to save settings {why:#?}")
            }
        });
    };

    view! {
        <button
            on:click=save_settings
            class="py-2 px-4 text-white bg-indigo-500 rounded hover:bg-indigo-600">
            "Save Changes"
        </button>
    }
}