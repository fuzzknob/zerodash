use super::{
    errors::Error,
    utils::{get_env, get_required_env},
    Result,
};
use serde::{Deserialize, Serialize, Serializer};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql, Surreal,
};
use tokio::fs;

pub type Db = Surreal<Client>;

pub type Datetime = sql::Datetime;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Id {
    pub id: sql::Id,
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.id.to_string())
    }
}

pub async fn initialize_database() -> Result<Db> {
    tracing::info!("Initializing database");
    let database_url = get_required_env("DATABASE_URL");
    let database_user = get_required_env("DATABASE_USER");
    let database_name = get_required_env("DATABASE_NAME");
    let database_password = get_required_env("DATABASE_PASSWORD");
    let db = Surreal::new::<Ws>(&database_url)
        .await
        .map_err(|_| Error::DatabaseConnectionError)?;
    db.signin(Root {
        username: &database_user,
        password: &database_password,
    })
    .await?;
    db.use_ns(&database_name).use_db(database_name).await?;
    Ok(db)
}

pub async fn run_migrations(db: &Db) -> Result<()> {
    tracing::info!("running migrations...");
    let migration_path = get_env("MIGRATION_DIRECTORY").unwrap_or("./migrations".to_string());
    if !fs::try_exists(&migration_path).await? {
        tracing::info!("migrations not found. skipping running migration...");
        return Ok(());
    }
    let mut migrations = fs::read_dir(migration_path).await?;
    while let Some(entry) = migrations.next_entry().await? {
        let content = fs::read_to_string(entry.path().display().to_string()).await?;
        db.query(content).await?;
    }
    tracing::info!("finished running migrations...");
    Ok(())
}
