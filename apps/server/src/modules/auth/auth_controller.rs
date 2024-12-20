use axum_extra::extract::cookie::{Cookie, SignedCookieJar};
use lunarus::prelude::*;
use serde_json::json;

use super::{
    auth_service::{AuthService, LoginResult, RegisterResult, TokenLoginResult},
    dto::{LoginDTO, RegisterDTO, TokenLoginDto},
    extractors::session_token::SessionToken,
};

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
        LoginResult::Ok(exchange_code) => res::json(json!({
            "exchange_token": exchange_code,
        })),
        _ => res::builder()
            .status(StatusCode::BAD_REQUEST)
            .message("Invalid credentials"),
    }
}

// TODO: find a way to make it more idiomatic
#[debug_handler]
pub async fn login_with_token(
    State(context): State<AppContext>,
    jar: SignedCookieJar,
    Query(exchange_token): Query<TokenLoginDto>,
) -> Result<impl IntoResponse> {
    exchange_token.validate()?;
    let result = AuthService::new(context.db)
        .login_with_token(exchange_token.token)
        .await?;
    let TokenLoginResult::Ok(session_token) = result else {
        return Err(Error::Unauthenticated);
    };
    let cookie = cookie::Cookie::build(("Authorization", format!("Bearer {}", session_token)))
        // .secure(true)
        .same_site(cookie::SameSite::None)
        .http_only(true)
        // .domain("http://localhost:3030")
        .path("/")
        .max_age(time::Duration::days(30))
        .build();
    let jar = jar.add(cookie);
    Ok((jar, res::message("Authenticated")))
}

#[debug_handler]
pub async fn logout(
    SessionToken(token): SessionToken,
    State(context): State<AppContext>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse> {
    AuthService::new(context.db)
        .delete_session_by_token(token)
        .await?;
    let jar = jar.remove(Cookie::from("Authorization"));
    Ok((jar, res::message("Logged out")))
}
