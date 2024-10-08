use super::{errors::LunarError, utils::get_required_env, Result};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub type Db = Surreal<Client>;

pub async fn initialize_database() -> Result<Surreal<Client>> {
    let database_url = get_required_env("DATABASE_URL");
    let database_user = get_required_env("DATABASE_USER");
    let database_name = get_required_env("DATABASE_NAME");
    let database_password = get_required_env("DATABASE_PASSWORD");
    let db = Surreal::new::<Ws>(&database_url)
        .await
        .map_err(|_| LunarError::DatabaseConnectionError)?;
    db.signin(Root {
        username: &database_user,
        password: &database_password,
    })
    .await
    .map_err(|_| LunarError::DatabaseCredentialError)?;
    db.use_ns(&database_name)
        .use_db(database_name)
        .await
        .map_err(|_| LunarError::DatabaseInitializationError)?;
    Ok(db)
}
