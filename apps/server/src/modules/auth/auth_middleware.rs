use axum::http::HeaderMap;
use axum_extra::extract::SignedCookieJar;
use lunarus::prelude::*;

use super::auth_service::AuthService;

pub async fn auth_middleware(
    State(db): State<Db>,
    jar: SignedCookieJar,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    let cookie = jar.get("Authorization");
    let token = if let Some(cookie) = cookie {
        dbg!(&cookie);
        cookie.value().to_owned()
    } else {
        let header = headers.get("Authorization").ok_or(Error::Unauthenticated)?;
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
    AuthService::new(db).check_token_validity(token).await?;
    let response = next.run(request).await;
    return Ok(response);
}
