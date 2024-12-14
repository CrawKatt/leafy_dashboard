use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub owner: bool,
    pub icon: Option<String>,
    pub permissions: Option<u64>
}

pub async fn get_user_guilds(access_token: &str) -> Result<Vec<Guild>, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://discord.com/api/users/@me/guilds")
        .bearer_auth(access_token)
        .send()
        .await?;

    response.json::<Vec<Guild>>().await
}