use super::auth_dto::RegisterDTO;
use super::auth_model::{AuthModel, AUTH_TABLE_NAME};
use crate::utils::hash;
use lunarus::prelude::*;

pub struct AuthService {
    db: Db,
}

impl AuthService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn register(&self, new_user: RegisterDTO) -> Result<RegisterResult> {
        let mut res = self
            .db
            .query("SELECT id from users where email = $email OR username = $username")
            .bind(("email", new_user.email.clone()))
            .bind(("username", new_user.username.clone()))
            .await?;
        let res: Vec<Record> = res.take(0)?;
        if !res.is_empty() {
            return Ok(RegisterResult::UserAlreadyExists);
        }
        let password_hash = hash::hash_password(new_user.password);
        let user: Option<AuthModel> = self
            .db
            .create(AUTH_TABLE_NAME)
            .content(RegisterDTO {
                name: new_user.name,
                email: new_user.email,
                username: new_user.username,
                password: password_hash,
            })
            .await?;
        Ok(RegisterResult::Ok(user.unwrap()))
    }
}

pub enum RegisterResult {
    Ok(AuthModel),
    UserAlreadyExists,
}
