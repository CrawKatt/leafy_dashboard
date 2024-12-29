use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn SaveChangesButton() -> impl IntoView {
    let save_settings = move |_| {
        let data_to_save = serde_json::json!({
            "admins": {
                "role": ["9876543210", "1234567890"]
            },
            "guild_id": "1056242001340809298",
            "forbidden": {
                "user": "1234567890",
                "role": "9876543210"
            },
            "time_out": {
                "time": "1 week"
            },
            "channels": {
                "welcome": "1234567890",
                "ooc": "9876543210",
                "logs": "1234567890",
                "exceptions": "9876543210"
            },
            "messages": {
                "welcome": "Welcome message",
                "time_out": "Timeout message",
                "warn": "Warn message"
            }
        });

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