use rust_embed::RustEmbed;
use serde_json::json;
use surf::mime;

#[derive(RustEmbed)]
#[folder = "./src/kibela/queries"]
struct Queries;

use super::graphql::parse_query;
use super::types::{Comment, CommentQueryRoot, GraphQLQueryRequest, Note, NoteQueryRoot, Result};

pub struct Client {
    token: String,
    team: String,
}

pub fn new(team: String, token: String) -> Client {
    Client { team, token }
}

impl Client {
    async fn exec_query(&self, query: &str, variables: serde_json::Value) -> Result<String> {
        let req = GraphQLQueryRequest {
            query: query.to_string(),
            variables,
        };
        surf::post(format!("https://{}.kibe.la/api/v1", self.team))
            .set_header("authorization", format!("Bearer {}", self.token))
            .body_string(serde_json::to_string(&req)?)
            .set_mime(mime::APPLICATION_JSON)
            .recv_string()
            .await
    }

    pub async fn note(&self, path: &str) -> Result<Note> {
        let query = Queries::get("note.gql").ok_or("can not get query file")?;
        let query: &str = std::str::from_utf8(query.as_ref())?;
        let res = self
            .exec_query(
                query,
                json!({
                    "path": path,
                }),
            )
            .await?;
        let res: NoteQueryRoot = parse_query(&res)?;
        Ok(res.note)
    }
    pub async fn comment(&self, comment_id: &str) -> Result<Comment> {
        let query = Queries::get("note_comment.gql").ok_or("can not get query file")?;
        let query: &str = std::str::from_utf8(query.as_ref())?;
        let res = self
            .exec_query(
                query,
                json!({
                    "id": base64::encode(format!("Comment/{}", comment_id)),
                }),
            )
            .await?;
        let res: CommentQueryRoot = parse_query(&res)?;
        Ok(res.comment)
    }
}
