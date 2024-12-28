use actix_web::{get, web, HttpResponse, Responder, Result};
use reqwest::Client;
use std::env;

use crate::api::error::BackEndError;
use crate::models::guild::DiscordUser;

#[get("/api/users/{guild_id}/{user_target}")]
pub async fn get_users(path: web::Path<(String, String)>) -> Result<impl Responder, BackEndError> {
    let access_token = env::var("BOT_TOKEN").expect("BOT TOKEN NOT FOUND");
    let (guild_id, user_target) = path.into_inner();

    let response = Client::new()
        .get(format!("https://discord.com/api/v10/guilds/{guild_id}/members/search?query={user_target}"))
        .header("Authorization", format!("Bot {access_token}"))
        .send()
        .await?;

    let users: Vec<DiscordUser> = response.json().await?;
    let http_response = HttpResponse::Ok().json(users);

    Ok(http_response)
}