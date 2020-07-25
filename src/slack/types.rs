use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum UnfurlBody {
    Attachment {
        author_link: String,
        author_name: String,
        title: String,
        title_link: String,
        text: String,
        ts: String,
        // 以下は static なので &str でもよい、変更がおきないため
        footer: String,
        footer_icon: String,
    },
}

#[derive(Debug, Serialize)]
pub struct Unfurl {
    pub channel: String,
    pub ts: String,
    // key = url
    pub unfurls: HashMap<String, UnfurlBody>,
}
