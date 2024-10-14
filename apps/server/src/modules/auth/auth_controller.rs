use super::{
    auth_dto::RegisterDTO,
    auth_service::{AuthService, RegisterResult},
};
use lunarus::prelude::*;

pub async fn register(
    State(db): State<Db>,
    JsonValidator(new_user): JsonValidator<RegisterDTO>,
) -> Result<impl IntoResponse> {
    let result = AuthService::new(db).register(new_user).await?;
    match result {
        RegisterResult::UserAlreadyExists => res::builder()
            .status(StatusCode::BAD_REQUEST)
            .message("User already exists"),
        _ => res::builder().message("Successfully registered user"),
    }
}
