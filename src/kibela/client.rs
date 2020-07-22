use rust_embed::RustEmbed;
use serde_json::json;
use surf::mime;

#[derive(RustEmbed)]
#[folder = "./src/kibela/queries"]
struct Queries;

use super::types::{Comment, Note, NoteQueryRoot, CommentQueryRoot, QueryResponse};

pub fn new(team: String, token: String) -> Client {
    Client {
        team,
        token,
    }
}

pub struct Client {
    token: String,
    team: String,
}

// TODO: めっちゃ適当
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

macro_rules! parse_query {
    ( $ty: ty, $val: expr ) => {
        {
            let res: QueryResponse<$ty> = serde_json::from_str(&$val)?;
            let res: Result<$ty> = match res {
                QueryResponse::Ok { data } => Ok(data),
                QueryResponse::Err { errors } => {
                    let errors: Vec<String> = errors.into_iter().map(|m| m.message).collect();
                    // into をやめたい (適当すぎる Result<T> を使うのやめたい)
                    Err(format!("GraphQL server returns error: {}", errors.as_slice().join(", ")).into())
                },
            };
            res
        }
    }
}

impl Client {
    pub async fn note(&self, path: &str) -> Result<Note> {
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
        let res = parse_query!(NoteQueryRoot, &res)?;
        Ok(res.note)
    }
    pub async fn comment(&self, comment_id: &str) -> Result<Comment> {
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

        let res = parse_query!(CommentQueryRoot, &res)?;
        Ok(res.comment)
    }
}
