use reqwest::Client;
use crate::models::user::Guild;

pub async fn get_user_guilds(access_token: &str) -> Result<Vec<Guild>, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://discord.com/api/users/@me/guilds")
        .bearer_auth(access_token)
        .send()
        .await?;

    response.json::<Vec<Guild>>().await
}