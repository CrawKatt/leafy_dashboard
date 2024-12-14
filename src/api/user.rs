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

    match get_user_guilds(&access_token).await {
        Ok(guilds) => {
            // Transforma Guild a Server
            let servers: Vec<Server> = guilds
                .into_iter()
                .map(|guild| Server {
                    id: guild.id.clone(),
                    name: guild.name,
                    owner: if guild.owner { "Owner".to_string() } else { "Member".to_string() },
                    icon: guild.icon.map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon)),
                })
                .collect();

            HttpResponse::Ok().json(servers)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}