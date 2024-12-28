use std::env;
use actix_web::{get, web, HttpResponse, Responder};
use reqwest::Client;
use crate::api::error::BackEndError;
use crate::models::guild::DiscordRole;

#[get("/api/roles/{guild_id}")]
pub async fn get_roles(guild_id: web::Path<String>) -> actix_web::Result<impl Responder, BackEndError> {
    let access_token = env::var("BOT_TOKEN").expect("BOT TOKEN NOT FOUND");
    let guild_id = guild_id.into_inner();
    let response = Client::new()
        .get(format!("https://discord.com/api/v10/guilds/{guild_id}/roles"))
        .header("Authorization", format!("Bot {}", access_token))
        .send()
        .await?;

    let channels: Vec<DiscordRole> = response.json().await?;
    let http_response = HttpResponse::Ok().json(channels);

    Ok(http_response)
}