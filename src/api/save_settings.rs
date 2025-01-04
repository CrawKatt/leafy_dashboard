use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::db::surreal::DB;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GuildData {
    admins: Admin,
    guild_id: String,
    forbidden: Forbidden,
    time_out: TimeOut,
    channels: Channels,
    messages: Messages,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Admin {
    role: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Forbidden {
    user: String,
    role: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TimeOut {
    time: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Channels {
    welcome: String,
    ooc: String,
    logs: String,
    exceptions: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Messages {
    welcome: String,
    time_out: String,
    warn: String
}

#[post("/api/save_settings")]
async fn save_settings(settings: web::Json<GuildData>) -> impl Responder {
    DB.use_ns("dashboard-namespace").use_db("dashboard").await.unwrap();
    let guild_id = settings.clone().guild_id;

    println!("Iniciando update: {settings:#?}");
    let current_data = get_current_data(guild_id.clone()).await.unwrap();
    let mut updates = Vec::new();
    println!("Updates 1: {:#?}", updates);

    // Comparar y actualizar solo los campos que han cambiado
    if settings.admins.role != current_data.admins.role {
        println!("Update admins: {:#?}", updates);
        updates.push(format!("admins.role = {}", serde_json::to_string(&settings.admins.role).unwrap()));
    }
    if settings.forbidden.user != current_data.forbidden.user {
        println!("Update forbidden_user: {:#?}", updates);
        updates.push(format!("forbidden.user = '{}'", settings.forbidden.user));
    }
    if settings.forbidden.role != current_data.forbidden.role {
        println!("Update forbidden_role: {:#?}", updates);
        updates.push(format!("forbidden.role = '{}'", settings.forbidden.role));
    }
    if settings.time_out.time != current_data.time_out.time {
        println!("Updates time_out_time: {:#?}", updates);
        updates.push(format!("time_out.time = '{}'", settings.time_out.time));
    }

    /*
    // Admins
    for (i, role) in settings.admins.role.iter().enumerate() {
        updates.push(format!("admins.role[{}] = '{}'", i, role));
    }

    // Forbidden
    updates.push(format!("forbidden.user = '{}'", settings.forbidden.user));
    updates.push(format!("forbidden.role = '{}'", settings.forbidden.role));

    // TimeOut
    updates.push(format!("time_out.time = '{}'", settings.time_out.time));

    // Channels
    updates.push(format!("channels.welcome = '{}'", settings.channels.welcome));
    updates.push(format!("channels.ooc = '{}'", settings.channels.ooc));
    updates.push(format!("channels.logs = '{}'", settings.channels.logs));
    updates.push(format!("channels.exceptions = '{}'", settings.channels.exceptions));

    // Messages
    updates.push(format!("messages.welcome = '{}'", settings.messages.welcome));
    updates.push(format!("messages.time_out = '{}'", settings.messages.time_out));
    updates.push(format!("messages.warn = '{}'", settings.messages.warn));
    */

    println!("Datos del vector: {updates:#?}");
    println!("Actualizando datos");

    // Construye una sola consulta SQL para todas las actualizaciones
    if !updates.is_empty() {
        let sql_query = format!("UPDATE guild_config SET {} WHERE guild_id = $guild_id", updates.join(", "));
        let _updated: Option<GuildData> = DB
            .query(sql_query)
            .bind(("guild_id", guild_id))
            .await
            .unwrap()
            .take(0)
            .unwrap();
        println!("Actualizados los campos: {:?}", updates);
    } else {
        println!("No se encontraron cambios para actualizar.");
    }

    println!("Datos recibidos: {:#?}", settings);

    HttpResponse::Ok().body("Ajustes guardados")
}

async fn get_current_data(guild_id: String) -> Option<GuildData> {
    let sql_query = "SELECT * FROM guild_config WHERE guild_id = $guild_id";
    let result: Option<GuildData> = DB
        .query(sql_query)
        .bind(("guild_id", guild_id.to_string()))
        .await
        .unwrap()
        .take(0)
        .unwrap();

    println!("Base de Datos: {result:#?}");
    result
}