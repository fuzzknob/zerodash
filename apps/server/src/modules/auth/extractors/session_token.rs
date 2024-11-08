use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::{cookie::Key, SignedCookieJar};
use lunarus::prelude::*;

pub struct SessionToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for SessionToken
where
    Key: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
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
        Ok(Self(token.to_owned()))
    }
}
