use actix_web::*;
use crate::models::user::Guild;

#[get("/api/servers")]
async fn get_servers(req: HttpRequest) -> impl Responder {
    // Lógica para devolver los servidores
    HttpResponse::Ok().json(vec![Guild {
        id: "1".to_string(),
        name: "Example Server".to_string(),
        owner: true,
        icon: None,
    }])
}

/*
#[get("/api/servers")]
pub async fn get_servers(req: HttpRequest) -> impl Responder {
    // Obtén el token del usuario (almacenado en una cookie segura)
    let access_token = req
        .cookie("access_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    if access_token.is_empty() {
        return HttpResponse::Unauthorized().body("Access token not found");
    }

    // Llama a la API de Discord para obtener los servidores del usuario
    match get_user_guilds(&access_token).await {
        Ok(servers) => HttpResponse::Ok().json(servers), // Retorna los servidores como JSON
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
*/