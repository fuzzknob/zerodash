use crate::modules::auth::email::verify_email::{VerifyEmailArgs, VerifyEmailMail};
use lunarus::prelude::*;

pub async fn index() -> impl IntoResponse {
    res::message("Zerodash api")
}

#[debug_handler]
pub async fn up() -> impl IntoResponse {
    let email_args = VerifyEmailArgs {
        name: "Gagan".to_string(),
        verification_link: "https://some-arbitary-link.com".to_string(),
    };
    res::message("Okay")
}
