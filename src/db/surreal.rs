use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client as SurrealClient, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<SurrealClient>> = LazyLock::new(Surreal::init);

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