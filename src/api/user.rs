use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};
use reqwest::Client;
use std::env;

use crate::services::discord::get_user_guilds;
use crate::api::error::BackEndError;
use crate::models::guild::{DiscordChannel, DiscordRole, DiscordServer, DiscordUser};

#[get("/api/servers")]
pub async fn get_servers(req: HttpRequest) -> Result<impl Responder, BackEndError> {
    let access_token = req
        .cookie("access_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    if access_token.is_empty() {
        return Ok(HttpResponse::Unauthorized().body("Access token not found"));
    }

    let guilds = get_user_guilds(&access_token).await?;
    let servers: Vec<DiscordServer> = guilds
        .into_iter()
        .filter(|guild| guild.owner || guild.permissions.unwrap_or(0) & 0x8 != 0) // 0x8 = ADMINISTRADOR
        .map(|guild| DiscordServer {
            guild_id: guild.id.clone(),
            name: guild.name,
            owner: if guild.owner { "Owner".to_string() } else { "Member".to_string() },
            icon: guild.icon.map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon)),
        })
        .collect();

    let http_response = HttpResponse::Ok().json(servers);
    Ok(http_response)
}

#[get("/api/servers/{guild_id}")]
pub async fn get_guild_id(
    guild_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let access_token = req
        .cookie("access_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    if access_token.is_empty() {
        return HttpResponse::Unauthorized().body("Access token not found");
    }

    let guild_id = guild_id.into_inner();
    let server = DiscordServer {
        guild_id: guild_id.clone(),
        name: format!("Server {}", guild_id),
        owner: "Owner".to_string(),
        icon: Some(format!(
            "https://cdn.discordapp.com/icons/{}/default.png",
            guild_id
        )),
    };

    HttpResponse::Ok().json(server)
}

#[get("/api/roles/{guild_id}")]
pub async fn get_roles(guild_id: web::Path<String>) -> Result<impl Responder, BackEndError> {
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

#[get("/api/channels/{guild_id}")]
pub async fn get_channels(guild_id: web::Path<String>) -> Result<impl Responder, BackEndError> {
    let access_token = env::var("BOT_TOKEN").expect("BOT TOKEN NOT FOUND");
    let guild_id = guild_id.into_inner();
    let response = Client::new()
        .get(format!("https://discord.com/api/v10/guilds/{guild_id}/channels"))
        .header("Authorization", format!("Bot {access_token}"))
        .send()
        .await?;

    let channels: Vec<DiscordChannel> = response.json().await?;
    let http_response = HttpResponse::Ok().json(channels);

    Ok(http_response)
}

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