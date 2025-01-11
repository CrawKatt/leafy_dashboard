use crate::models::guild::GuildData;
use once_cell::sync::Lazy;
use serde_json::Value;
use surrealdb::engine::remote::ws::{Client as SurrealClient, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::opt::PatchOp;
use surrealdb::Surreal;

pub static DB: Lazy<Surreal<SurrealClient>> = Lazy::new(Surreal::init);

pub async fn setup_db() {
    let database_password = dotenv::var("DATABASE_PASSWORD").expect("MISSING SURREAL_PASSWORD");
    DB.connect::<Ws>("0.0.0.0:8000").await.unwrap_or_else(|why| {
        panic!("Could not connect to database: {why}")
    });

    DB.signin(Root {
        username: "root",
        password: &database_password
    }).await.expect("Could not sign in");
}

pub async fn get_guild_config(guild_id: &String) -> Option<GuildData> {
    DB.use_ns("dashboard-namespace").use_db("dashboard").await.unwrap();
    let result: Option<GuildData> = DB
        .select(("guild_config", guild_id))
        .await
        .unwrap_or_default();

    result
}

pub async fn update_guild_configs(guild_id: &String, path: &str, value: &Value) -> Option<GuildData> {
    DB.use_ns("dashboard-namespace").use_db("dashboard").await.unwrap();
    let result: Option<GuildData> = DB
        .update(("guild_config", guild_id))
        .patch(PatchOp::replace(path, value))
        .await
        .unwrap_or_default();

    result
}

pub async fn add_guild_config(guild_id: &String, guild_config: GuildData) -> Option<GuildData> {
    DB.use_ns("dashboard-namespace").use_db("dashboard").await.unwrap();
    let results = DB
        .create(("guild_config", guild_id))
        .content(guild_config)
        .await;

    results.unwrap_or_default()
}