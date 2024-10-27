use crate::template::TemplateBuilder;
use crate::{
    utils::{get_env, get_required_env},
    Error, Result,
};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};

pub struct Message {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
}

pub async fn send_email(message: Message) -> Result<()> {
    let mailtrap_host = get_required_env("MAILTRAP_HOST");
    let mailtrap_token = get_required_env("MAILTRAP_API_TOKEN");
    let post_url = format!("{}/api/send/3233785", mailtrap_host);
    let to_emails: Vec<Value> = message
        .to
        .iter()
        .map(|email| json!({"email": email}))
        .collect();
    let email_payload = json!({
        "from": {"email" : message.from},
        "to": to_emails,
        "subject": message.subject,
        "html": message.body,
    });
    let response = Client::new()
        .post(post_url)
        .header("Content-Type", "application/json")
        .header("Api-Token", mailtrap_token)
        .body(email_payload.to_string())
        .send()
        .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        let error = response.text().await?;
        Err(Error::EmailSendError(error))
    }
}

#[derive(Debug)]
pub struct Mail {
    pub from: String,
    pub subject: String,
    pub body: String,
}

impl Mail {
    pub fn new() -> Self {
        let default_from_email =
            get_env("MAIL_DEFAULT_FROM_ADDRESS").unwrap_or("hello@example.com".to_string());
        Self {
            from: default_from_email,
            subject: "".to_string(),
            body: "".to_string(),
        }
    }

    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_string();
        self
    }

    pub fn from(mut self, from: &str) -> Self {
        self.from = from.to_string();
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn template<T: Serialize>(mut self, template: &str, data: Option<T>) -> Result<Self> {
        let body = TemplateBuilder::new(template.to_string(), data).build()?;
        self.body = body;
        Ok(self)
    }

    pub async fn send(&self, email: String) -> Result<()> {
        self.send_email(vec![email]).await
    }

    pub async fn send_many(&self, emails: Vec<String>) -> Result<()> {
        self.send_email(emails).await
    }

    async fn send_email(&self, emails: Vec<String>) -> Result<()> {
        send_email(Message {
            from: self.from.clone(),
            to: emails,
            subject: self.subject.clone(),
            body: self.body.clone(),
        })
        .await
    }
}

impl Default for Mail {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Mailer<T> {
    fn build(args: T) -> Result<Mail>;
}
