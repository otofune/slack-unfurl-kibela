use rust_embed::RustEmbed;
use serde_json::json;

#[derive(RustEmbed)]
#[folder = "./src/kibela/queries"]
struct Queries;

use super::{
    graphql::{parse_query, GraphQLQueryRequest},
    types::{CommentAndNote, Note, NoteQueryRoot},
};
use crate::types::Result;

#[derive(Clone)]
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
            .body_json(&req)?
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
    pub async fn comment_and_note(&self, comment_id: &str, path: &str) -> Result<CommentAndNote> {
        let query = Queries::get("comment_and_note.gql").ok_or("can not get query file")?;
        let query: &str = std::str::from_utf8(query.as_ref())?;
        let res = self
            .exec_query(
                query,
                json!({
                    "comment_id": base64::encode(format!("Comment/{}", comment_id)),
                    "path": path,
                }),
            )
            .await?;
        parse_query::<CommentAndNote>(&res)
    }
}
