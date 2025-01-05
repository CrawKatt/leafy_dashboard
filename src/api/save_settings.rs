use crate::db::surreal::{add_guild_config, get_guild_config, update_guild_configs};
use crate::models::guild::GuildData;
use actix_web::{put, web, HttpResponse, Responder};

#[put("/api/save_settings")]
async fn save_settings(settings: web::Json<GuildData>) -> impl Responder {
    let guild_id = settings.guild_id.clone();

    let guild = get_guild_config(&guild_id).await;
    println!("Guild: {:#?}", guild.clone());

    if guild.is_none() {
        add_guild_config(settings.into_inner()).await;
        return HttpResponse::Created().body("Ajustes Guardados")
    }

    let path = "forbidden/role";
    let value = "hola_mundo";
    update_guild_configs(guild_id, path, value).await;

    HttpResponse::Ok().body("Ajustes Actualizados")
}