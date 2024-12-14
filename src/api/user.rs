use actix_web::{get, HttpRequest, HttpResponse, Responder};
use crate::frontend::components::server_card::Server;
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

    let servers: Vec<Server> = guilds
        .into_iter()
        .filter(|guild| guild.owner || guild.permissions.unwrap_or(0) & 0x8 != 0) // 0x8 = ADMINISTRADOR
        .map(|guild| Server {
            id: guild.id.clone(),
            name: guild.name,
            owner: if guild.owner { "Owner".to_string() } else { "Member".to_string() },
            icon: guild.icon.map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon)),
        })
        .collect();

    HttpResponse::Ok().json(servers)
}