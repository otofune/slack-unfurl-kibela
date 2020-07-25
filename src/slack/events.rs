use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Link {
    pub domain: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EventCallback {
    LinkShared { channel: String, links: Vec<Link> },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    UrlVerification { challenge: String },
    EventCallback { event: EventCallback },
}
