#[derive(Clone)]
pub struct Client {
    authorization: String,
}

pub fn new(bot_token: String) -> Client {
    Client {
        authorization: format!("Bearer {}", bot_token),
    }
}

use super::types::Unfurl;
use crate::types::Result;

impl Client {
    pub async fn unfurl(&self, unfurl: Unfurl) -> Result<()> {
        let mut res = surf::post("https://slack.com/api/chat.unfurl")
            .body_json(&unfurl)?
            .set_header("Authorization", &self.authorization)
            .await?;
        let body = res
            .body_string()
            .await
            .unwrap_or("<can not retrive body>".into());
        if !res.status().is_success() {
            return Err(format!("Slack returns {} error: {}", res.status().as_str(), body,).into());
        }
        Ok(())
    }
}
