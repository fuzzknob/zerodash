use lunarus::{mail::Mail, Result};
use serde::Serialize;

static TEMPLATE: &str = "
Dear {{name}},

Thank you for creating an account with us. To complete the registration process, we need to verify your email address.

Please click the link below to verify your email address:

<a href=\"{{verification_link}}\">Verify Email Address</a>

If you have any issues with the verification link, you can copy and paste the following URL into your browser:

<a href=\"{{verification_link}}\">{{verification_link}}</a>

Once you've verified your email address, you'll be able to access all the features of our platform.

Best regards,
Gagan from zerodash
";

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
