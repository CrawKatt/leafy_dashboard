use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn SaveChangesButton(
    admin_roles: ReadSignal<Vec<String>>,
    guild_id: String,
    forbidden_user: ReadSignal<String>,
    forbidden_role: ReadSignal<String>,
    timeout_time: ReadSignal<String>,
    welcome_channel: ReadSignal<String>,
    ooc_channel: ReadSignal<String>,
    logs_channel: ReadSignal<String>,
    exceptions_channel: ReadSignal<String>,
    welcome_message: ReadSignal<String>,
    timeout_message: ReadSignal<String>,
    warn_message: ReadSignal<String>
) -> impl IntoView {
    let save_settings = move |_| {
        let data_to_save = serde_json::json!({
            "admins": {
                "role": admin_roles.get()
            },
            "guild_id": guild_id,
            "forbidden": {
                "user": forbidden_user.get(),
                "role": forbidden_role.get()
            },
            "time_out": {
                "time": timeout_time.get()
            },
            "channels": {
                "welcome": welcome_channel.get(),
                "ooc": ooc_channel.get(),
                "logs": logs_channel.get(),
                "exceptions": exceptions_channel.get()
            },
            "messages": {
                "welcome": welcome_message.get(),
                "time_out": timeout_message.get(),
                "warn": warn_message.get()
            }
        });

        log!("Datos para enviar: {:#?}", data_to_save);

        spawn_local(async move {
            let client = reqwest::Client::new();
            let response = client
                .post("http://localhost:3000/api/save_settings")
                .json(&data_to_save)
                .send()
                .await;

            match response {
                Ok(_) => log::info!("Settins saved successfully"),
                Err(why) => log::error!("Failed to save settings {:#?}", why)
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