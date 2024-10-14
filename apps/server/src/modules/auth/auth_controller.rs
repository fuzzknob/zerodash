use super::{
    auth_dto::{LoginDTO, RegisterDTO},
    auth_service::{AuthService, LoginResult, RegisterResult},
};
use lunarus::prelude::*;

pub async fn register(
    State(db): State<Db>,
    JsonValidator(new_user): JsonValidator<RegisterDTO>,
) -> Result<Response> {
    let result = AuthService::new(db).register(new_user).await?;
    match result {
        RegisterResult::UserAlreadyExists => res::builder()
            .status(StatusCode::BAD_REQUEST)
            .message("User already exists"),
        _ => res::builder().message("Successfully registered user"),
    }
}

pub async fn login(
    State(db): State<Db>,
    JsonValidator(credentials): JsonValidator<LoginDTO>,
) -> Result<Response> {
    let result = AuthService::new(db).login(credentials).await?;
    match result {
        LoginResult::Ok(session) => res::json(session),
        _ => res::builder()
            .status(StatusCode::BAD_REQUEST)
            .message("Invalid credentials"),
    }
}
