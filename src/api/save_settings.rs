use crate::db::surreal::{add_guild_config, get_guild_config, update_guild_configs};
use crate::models::guild::GuildData;
use actix_web::{put, web, HttpResponse, Responder};
use serde_json::Value;

#[put("/api/save_settings")]
async fn save_settings(settings: web::Json<Value>) -> impl Responder {
    let guild_id = settings
        .get("guild_id")
        .and_then(|value| value.as_str())
        .map(String::from)
        .unwrap_or_default();

    let guild = get_guild_config(&guild_id).await;

    if guild.is_none() {
        let Some(config) = settings
            .get("guild_config")
            .and_then(|value| serde_json::from_value::<GuildData>(value.clone()).ok())
        else {
            return HttpResponse::BadRequest().body("Datos de configuración inválidos");
        };

        add_guild_config(&guild_id, config).await;
        return HttpResponse::Created().body("Ajustes Guardados");
    }

    let Some(patch) = settings.get("patch").and_then(Value::as_array) else {
        return HttpResponse::BadRequest().body("Datos de actualización inválidos")
    };

    for operation in patch {
        if let (Some(path), Some(value)) = (
            operation.get("path").and_then(Value::as_str),
            operation.get("value")
        ) {
            if !value.is_null() &&
                !(path == "admins/role" && value.as_array().map_or(false, |arr| arr.is_empty())) &&
                !(value.is_string() && value.as_str().map_or(false, |s| s.is_empty())) {
                update_guild_configs(&guild_id, path, &value).await;
            }
        }
    }

    HttpResponse::Ok().body("Ajustes Actualizados")
}