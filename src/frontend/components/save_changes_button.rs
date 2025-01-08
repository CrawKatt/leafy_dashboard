use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::json;
use crate::models::guild::{Admin, Channels, Forbidden, GuildData, Messages, PatchOperation, TimeOut};

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
        let guild_id = guild_id.clone();
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

        let operations = vec![
            PatchOperation {
                op: "replace".to_string(),
                path: "admins/role".to_string(),
                value: json!(admin_roles.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "forbidden/user".to_string(),
                value: json!(forbidden_user.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "forbidden/role".to_string(),
                value: json!(forbidden_role.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "time_out/time".to_string(),
                value: json!(timeout_time.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "channels/welcome".to_string(),
                value: json!(welcome_channel.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "channels/ooc".to_string(),
                value: json!(ooc_channel.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "channels/logs".to_string(),
                value: json!(logs_channel.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "channels/exceptions".to_string(),
                value: json!(exceptions_channel.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "messages/welcome".to_string(),
                value: json!(welcome_message.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "messages/time_out".to_string(),
                value: json!(timeout_message.get_untracked()),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "messages/warn".to_string(),
                value: json!(warn_message.get_untracked()),
            },
        ];

        spawn_local(async move {
            let client = reqwest::Client::new();
            let check_config = client
                .get(format!("http://localhost:3000/api/get_settings/{guild_id}"))
                .send()
                .await;

            let config_exists = match check_config {
                Ok(resp) if resp.status().is_success() => resp.json::<bool>().await.unwrap_or(false),
                _ => false
            };

            if config_exists {
                let data_to_update = json!({
                    "guild_id": guild_id,
                    "patch": operations
                });

                let response = client
                    .put("http://localhost:3000/api/save_settings")
                    .json(&data_to_update)
                    .send()
                    .await;

                match response {
                    Ok(code) => log!("Settins saved successfully {code:#?}"),
                    Err(why) => error!("Failed to save settings {why:#?}")
                }
            } else {
                let response = client
                    .put("http://localhost:3000/api/save_settings")
                    .json(&data_to_save)
                    .send()
                    .await;

                match response {
                    Ok(code) => log!("Settins saved successfully {code:#?}"),
                    Err(why) => error!("Failed to save settings {why:#?}")
                }
            }
        });
    };

    view! {
        <button
            on:click=save_settings
            class="py-2 px-4 text-white bg-indigo-500 rounded hover:bg-indigo-600"
        >
            "Save Changes"
        </button>
    }
}