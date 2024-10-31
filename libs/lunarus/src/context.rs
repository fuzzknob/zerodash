use super::database::Db;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: Surreal<Client>,
    pub key: Key,
}

impl FromRef<AppContext> for Db {
    fn from_ref(state: &AppContext) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppContext> for Key {
    fn from_ref(state: &AppContext) -> Self {
        state.key.clone()
    }
}
