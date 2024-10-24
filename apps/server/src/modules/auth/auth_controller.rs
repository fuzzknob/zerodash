use super::{
    auth_service::{AuthService, LoginResult, RegisterResult},
    dto::{LoginDTO, RegisterDTO},
};
use lunarus::prelude::*;

pub async fn register(State(db): State<Db>, Json(new_user): Json<RegisterDTO>) -> Result<Response> {
    new_user.validate()?;
    let auth_service = AuthService::new(db);
    let result = auth_service.register(new_user).await?;
    if let RegisterResult::UserAlreadyExists = result {
        return res::builder()
            .status(StatusCode::BAD_REQUEST)
            .message("User already exists");
    }
    if let RegisterResult::Ok(user) = result {
        tokio::spawn(async move {
            if let Err(error) = auth_service.user_post_registration_setup(user).await {
                dbg!(error);
            }
        });
    }
    res::message("Successfully registered user")
}

pub async fn login(State(db): State<Db>, Json(credentials): Json<LoginDTO>) -> Result<Response> {
    credentials.validate()?;
    let result = AuthService::new(db).login(credentials).await?;
    match result {
        LoginResult::Ok(session) => res::json(session),
        _ => res::builder()
            .status(StatusCode::BAD_REQUEST)
            .message("Invalid credentials"),
    }
}
