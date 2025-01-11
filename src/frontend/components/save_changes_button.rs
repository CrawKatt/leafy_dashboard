use leptos::logging::{error, log};
use leptos::prelude::*;
use leptos::task::spawn_local;
use reqwest::Client;
use serde_json::json;
use crate::frontend::global_state::GlobalState;
use crate::models::guild::{Admin, Channels, Forbidden, GuildData, Messages, PatchOperation, TimeOut};

#[component]
pub fn SaveChangesButton(guild_id: String) -> impl IntoView {
    let global_state = use_context::<GlobalState>().expect("GlobalState not provided");
    let save_settings = move |_| {
        let guild_id = guild_id.clone();
        let global_state = global_state.clone();
        let data_to_save = create_data(&global_state);
        let operations = create_operations(&global_state);

        spawn_local(async move {
            let client = Client::new();
            let config_exists = get_config_data(&client, &guild_id).await;

            if config_exists {
                update_data(guild_id, operations, client).await;
            } else {
                save_data(data_to_save, guild_id, client).await;
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

fn create_data(global_state: &GlobalState) -> GuildData {
    GuildData::builder()
        .admins(Admin::builder()
            .role(global_state.admin_roles.get())
            .build()
        )
        .forbidden(Forbidden::builder()
            .user(global_state.forbidden_user.get())
            .role(global_state.forbidden_role.get())
            .build()
        )
        .time_out(TimeOut::builder()
            .time(global_state.timeout_time.get())
            .build()
        )
        .channels(Channels::builder()
            .welcome(global_state.welcome_channel.get())
            .ooc(global_state.ooc_channel.get())
            .logs(global_state.logs_channel.get())
            .exceptions(global_state.exceptions_channel.get())
            .build()
        )
        .messages(Messages::builder()
            .welcome(global_state.welcome_message.get())
            .time_out(global_state.timeout_message.get())
            .warn(global_state.warn_message.get())
            .build()
        )
        .build()
}

fn create_operations(global_state: &GlobalState) -> Vec<PatchOperation> {
    vec![
        PatchOperation {
            op: "replace".to_string(),
            path: "admins/role".to_string(),
            value: json!(global_state.admin_roles.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "forbidden/user".to_string(),
            value: json!(global_state.forbidden_user.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "forbidden/role".to_string(),
            value: json!(global_state.forbidden_role.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "time_out/time".to_string(),
            value: json!(global_state.timeout_time.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "channels/welcome".to_string(),
            value: json!(global_state.welcome_channel.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "channels/ooc".to_string(),
            value: json!(global_state.ooc_channel.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "channels/logs".to_string(),
            value: json!(global_state.logs_channel.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "channels/exceptions".to_string(),
            value: json!(global_state.exceptions_channel.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "messages/welcome".to_string(),
            value: json!(global_state.welcome_message.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "messages/time_out".to_string(),
            value: json!(global_state.timeout_message.get_untracked()),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "messages/warn".to_string(),
            value: json!(global_state.warn_message.get_untracked()),
        },
    ]
}

async fn update_data(
    guild_id: String,
    operations: Vec<PatchOperation>,
    client: Client
) {
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
}

async fn save_data(data_to_save: GuildData, guild_id: String, client: Client) {
    let data_to_save = json!({
        "guild_id": guild_id,
        "guild_config": data_to_save
    });
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

async fn get_config_data(client: &Client, guild_id: &String) -> bool {
    let check_config = client
        .get(format!("http://localhost:3000/api/get_settings/{guild_id}"))
        .send()
        .await;

    match check_config {
        Ok(resp) if resp.status().is_success() => resp.json::<bool>().await.unwrap_or(false),
        _ => false
    }
}