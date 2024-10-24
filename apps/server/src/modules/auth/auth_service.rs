use super::dto::{LoginDTO, RegisterDTO};
use super::model::SessionModel;
use crate::modules::boards::board_service::BoardService;
use crate::modules::spaces::space_service::SpaceService;
use crate::modules::users::model::UserModel;
use crate::utils::{self, hash};
use lunarus::prelude::*;
use surrealdb::sql::Datetime;

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
        let password_hash = hash::hash_password(new_user.password.unwrap());
        let user: Option<UserModel> = self
            .db
            .create(UserModel::TABLE_NAME)
            .content(RegisterDTO {
                name: new_user.name,
                email: new_user.email,
                username: new_user.username,
                password: Some(password_hash),
            })
            .await?;
        Ok(RegisterResult::Ok(user.unwrap()))
    }

    pub async fn login(&self, credential: LoginDTO) -> Result<LoginResult> {
        let identity = credential.identity.unwrap();
        let password = credential.password.unwrap();
        let user: Option<UserModel> = if utils::is_email(&identity) {
            let mut result = self
                .db
                .query("SELECT * FROM type::table($table) WHERE email = $email")
                .bind(("table", UserModel::TABLE_NAME))
                .bind(("email", identity))
                .await?;
            result.take(0)?
        } else {
            let mut result = self
                .db
                .query("SELECT * FROM type::table($table) WHERE username = $username")
                .bind(("table", UserModel::TABLE_NAME))
                .bind(("username", identity))
                .await?;
            result.take(0)?
        };
        let user = match user {
            Some(user) => user,
            None => return Ok(LoginResult::UserNotFound),
        };
        let is_password_valid = hash::verify_password(&password, &user.password);
        if !is_password_valid {
            return Ok(LoginResult::InvalidCredentials);
        }
        let expiration = chrono::Utc::now() + chrono::Duration::days(30);
        let session: Option<SessionModel> = self
            .db
            .query("CREATE type::table($table) set token = type::string($session_token), user = type::thing($user_id), expiration = $expiration")
            .bind(("table", SessionModel::TABLE_NAME))
            .bind(("session_token", hash::get_unique_random_hash()))
            .bind(("user_id", format!("{}:{}", UserModel::TABLE_NAME, user.id.to_string())))
            .bind(("expiration", Datetime::from(expiration)))
            .await?
            .take(0)?;
        Ok(LoginResult::Ok(session.unwrap()))
    }

    pub async fn check_token_validity(self, token: &str) -> Result<()> {
        let session: Option<SessionModel> = self
            .db
            .query("SELECT * FROM type::table($table) where token = $session_token;")
            .bind(("table", SessionModel::TABLE_NAME))
            .bind(("session_token", token.to_string()))
            .await?
            .take(0)?;
        let session = session.ok_or(Error::Unauthenticated)?;
        if session.is_valid() {
            Ok(())
        } else {
            Err(Error::Unauthenticated)
        }
    }

    pub async fn user_post_registration_setup(&self, user: UserModel) -> Result<()> {
        let space = SpaceService::new(self.db.clone())
            .create_default_space(user)
            .await?;
        BoardService::new(self.db.clone())
            .create_default_board(space.id.to_string())
            .await?;
        Ok(())
    }
}

pub enum LoginResult {
    Ok(SessionModel),
    UserNotFound,
    InvalidCredentials,
}

pub enum RegisterResult {
    Ok(UserModel),
    UserAlreadyExists,
}
