use std::env;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use reqwest::Client;
use crate::frontend::components::server_card::{DiscordChannel, DiscordRole, DiscordServer};
use crate::services::discord::get_user_guilds;

#[get("/api/servers")]
pub async fn get_servers(req: HttpRequest) -> impl Responder {
    let access_token = req
        .cookie("access_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    if access_token.is_empty() {
        return HttpResponse::Unauthorized().body("Access token not found");
    }

    let user_guilds = get_user_guilds(&access_token).await;
    let Ok(guilds) = user_guilds else {
        return HttpResponse::InternalServerError().body("Ocurrió un error al obtener los servidores del usuario")
    };

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

    HttpResponse::Ok().json(servers)
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
pub async fn get_roles(guild_id: web::Path<String>) -> impl Responder {
    let access_token = env::var("BOT_TOKEN").expect("BOT TOKEN NOT FOUND");
    let guild_id = guild_id.into_inner();
    let client = Client::new();

    match client
        .get(format!("https://discord.com/api/v10/guilds/{guild_id}/roles"))
        .header("Authorization", format!("Bot {}", access_token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let roles: Vec<DiscordRole> = match response.json().await {
                    Ok(roles) => roles,
                    Err(_) => {
                        return HttpResponse::InternalServerError().body("Failed to parse roles")
                    },
                };
                HttpResponse::Ok().json(roles)
            } else {
                HttpResponse::Unauthorized().body("Failed to fetch roles from Discord")
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Request to Discord API failed")
        }
    }
}

#[get("/api/channels/{guild_id}")]
pub async fn get_channels(guild_id: web::Path<String>) -> impl Responder {
    let access_token = env::var("BOT_TOKEN").expect("BOT TOKEN NOT FOUND");
    let guild_id = guild_id.into_inner();
    let client = Client::new();

    match client
        .get(format!("https://discord.com/api/v10/guilds/{guild_id}/channels"))
        .header("Authorization", format!("Bot {}", access_token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let channels: Vec<DiscordChannel> = match response.json().await {
                    Ok(roles) => roles,
                    Err(_) => {
                        return HttpResponse::InternalServerError().body("Failed to parse roles")
                    },
                };
                HttpResponse::Ok().json(channels)
            } else {
                HttpResponse::Unauthorized().body("Failed to fetch roles from Discord")
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Request to Discord API failed")
        }
    }
}