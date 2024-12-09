use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::Result as SurrealResult;

pub async fn setup_db() -> SurrealResult<Surreal<Client>> {
    let db = Surreal::new::<Ws>("0.0.0.0:8000").await?;
    db.connect::<Ws>("0.0.0.0:8000").await?;
    db.signin(Root {
        username: "root",
        password: "061020",
    }).await?;

    db.use_ns("dashboard-namespace").use_db("dashboard").await?;
    Ok(db)
}