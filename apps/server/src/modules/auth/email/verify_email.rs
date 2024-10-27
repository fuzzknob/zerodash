use lunarus::{mail::Mail, Result};
use serde::Serialize;

static TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
    <body style="font-family: Inter, Arial, sans-serif">
        <p>Hi {{name}},</p>
        <p>Thank you for creating an account with us. To complete the registration process, we need to verify your email address.</p>
        <p>Please click the link below to verify your email address:</p>
        <a href="{{verification_link}}">Verify Email Address</a>
        <p>If you have any issues with the verification link, you can copy and paste the following URL into your browser:</p>
        <a href="{{verification_link}}">{{verification_link}}</a>
        <p>Once you've verified your email address, you'll be able to access all the features of our platform.</p>
        <p>
            Kind regards, <br />
            Gagan from <del>Zero</del>
        </p>
    </body>
</html>
"#;

#[derive(Debug, Serialize)]
pub struct VerifyEmailArgs {
    pub name: String,
    pub verification_link: String,
}

pub struct VerifyEmailMail;

impl VerifyEmailMail {
    pub fn build(args: VerifyEmailArgs) -> Result<Mail> {
        Mail::new()
            .subject("Before we begin. Please verify your email")
            .template(TEMPLATE, Some(args))
    }
}
