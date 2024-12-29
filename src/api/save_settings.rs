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
    let guild_id = "1056242001340809298";

    let updates = vec![
        ("admins", serde_json::to_string(&settings.admins).unwrap()),
        ("guild_id", serde_json::to_string(&settings.guild_id).unwrap()),
        ("forbidden", serde_json::to_string(&settings.forbidden).unwrap()),
        ("time_out", serde_json::to_string(&settings.time_out).unwrap()),
        ("channels", serde_json::to_string(&settings.channels).unwrap()),
        ("messages", serde_json::to_string(&settings.messages).unwrap()),
    ];

    println!("Actualizando datos");

    for (field_name, value) in updates {
        println!("Field name: {field_name}");
        println!("Value: {value}");
        let sql_query = format!("UPDATE guild_config SET {field_name} = $value WHERE guild_id = $guild_id");
        let _updated: Option<GuildData> = DB
            .query(sql_query)
            .bind(("value", value))
            .bind(("guild_id", guild_id))
            .await
            .unwrap()
            .take(0)
            .unwrap();
    }

    println!("Datos recibidos: {:#?}", settings);

    HttpResponse::Ok().body("Ajustes guardados")
}