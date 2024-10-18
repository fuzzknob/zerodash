use axum::http::HeaderMap;
use lunarus::prelude::*;

use super::auth_service::AuthService;

pub async fn auth_middleware(
    State(db): State<Db>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let token = headers.get("Authorization").ok_or(Error::Unauthenticated)?;
    let token = token.to_str().map_err(|_| Error::Unauthenticated)?;
    let token = token
        .strip_prefix("Bearer ")
        .ok_or(Error::Unauthenticated)?;
    if token.is_empty() {
        return Err(Error::Unauthenticated);
    }
    AuthService::new(db).check_token_validity(token).await?;
    let response = next.run(request).await;
    return Ok(response);
}
