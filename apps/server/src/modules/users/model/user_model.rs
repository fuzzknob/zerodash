use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::{cookie::Key, SignedCookieJar};
use lunarus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::auth::auth_service::AuthService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Id,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email_verified_at: Option<Datetime>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

impl UserModel {
    pub const TABLE_NAME: &str = "users";

    pub fn first_name(&self) -> String {
        let split: Vec<&str> = self.name.split_whitespace().collect();
        split.get(0).unwrap().to_string()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserModel
where
    Db: FromRef<S>,
    Key: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let db = Db::from_ref(state);
        let key = Key::from_ref(state);
        let jar = SignedCookieJar::from_headers(&parts.headers, key);
        let cookie = jar.get("Authorization");
        let token = if let Some(cookie) = cookie {
            cookie.value().to_string()
        } else {
            let header = parts
                .headers
                .get("Authorization")
                .ok_or(Error::Unauthenticated)?;
            header
                .to_str()
                .map_err(|_| Error::Unauthenticated)?
                .to_owned()
        };
        let token = token
            .strip_prefix("Bearer ")
            .ok_or(Error::Unauthenticated)?;
        if token.is_empty() {
            return Err(Error::Unauthenticated);
        }
        AuthService::new(db)
            .get_user_from_session(token.to_string())
            .await
    }
}
