use rust_embed::RustEmbed;
use serde_json::json;
use surf::mime;

#[derive(RustEmbed)]
#[folder = "./src/kibela/queries"]
struct Queries;

use super::types::{CommentQuerySuccessfulResponse, NoteQuerySuccessfulResponse};

pub fn new(team: &str, token: &str) -> Client {
    Client {
        team: String::from(team),
        token: String::from(token),
    }
}

pub struct Client {
    token: String,
    team: String,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

impl Client {
    pub async fn note(&self, path: &str) -> Result<NoteQuerySuccessfulResponse> {
        let query = Queries::get("note.gql").ok_or("can not get query file")?;
        let query: &str = std::str::from_utf8(query.as_ref())?;
        let req = json!({
            "query": query,
            "variables": {
                "path": path,
            }
        });
        let req = serde_json::to_string(&req)?;
        let res = surf::post(format!("https://{}.kibe.la/api/v1", self.team))
            .set_header("authorization", format!("Bearer {}", self.token))
            .body_string(req)
            .set_mime(mime::APPLICATION_JSON)
            .recv_string()
            .await?;
        Ok(serde_json::from_str(&res)?)
    }
    pub async fn comment(&self, comment_id: &str) -> Result<CommentQuerySuccessfulResponse> {
        let query = Queries::get("note_comment.gql").ok_or("can not get query file")?;
        let query: &str = std::str::from_utf8(query.as_ref())?;
        let req = json!({
            "query": query,
            "variables": {
                "id": base64::encode(format!("Comment/{}", comment_id)),
            }
        });
        let req = serde_json::to_string(&req)?;
        let res = surf::post(format!("https://{}.kibe.la/api/v1", self.team))
            .set_header("authorization", format!("Bearer {}", self.token))
            .body_string(req)
            .set_mime(mime::APPLICATION_JSON)
            .recv_string()
            .await?;
        Ok(serde_json::from_str(&res)?)
    }
}
