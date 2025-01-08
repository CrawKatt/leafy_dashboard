use actix_web::{get, web, HttpResponse, Responder};
use crate::db::surreal::get_guild_config;

#[get("/api/get_settings/{guild_id}")]
async fn get_settings(settings: web::Path<String>) -> impl Responder {
    let guild_id = settings.into_inner();
    let guild_config = get_guild_config(&guild_id).await.is_some();
    HttpResponse::Ok().json(guild_config)
}