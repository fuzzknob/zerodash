use super::database::Db;
use axum::extract::FromRef;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: Surreal<Client>,
}

impl FromRef<AppContext> for Db {
    fn from_ref(input: &AppContext) -> Self {
        input.db.clone()
    }
}
