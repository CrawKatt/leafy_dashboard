use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::frontend::components::server_card::DiscordServer;
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
    // Verifica si existe un token de acceso
    let access_token = req
        .cookie("access_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    if access_token.is_empty() {
        return HttpResponse::Unauthorized().body("Access token not found");
    }

    // Aquí puedes agregar lógica para consultar los datos del servidor específico usando el guild_id.
    // Ejemplo básico de respuesta:
    let guild_id = guild_id.into_inner();

    // Simula obtener datos del servidor, podrías reemplazar con una consulta a un servicio o BD.
    let server = DiscordServer {
        guild_id: guild_id.clone(),
        name: format!("Server {}", guild_id),
        owner: "Owner".to_string(),
        icon: Some(format!(
            "https://cdn.discordapp.com/icons/{}/default.png",
            guild_id
        )),
    };

    // Devuelve los datos del servidor
    HttpResponse::Ok().json(server)
}