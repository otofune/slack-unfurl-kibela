use serde::{Deserialize, Serialize};

pub type ID = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAvatarImage {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub account: String,
    pub id: ID,
    #[serde(rename = "avatarImage")]
    pub avatar_image: UserAvatarImage,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: ID,
    pub title: String,
    pub url: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub summary: String,
    pub author: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: ID,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub summary: String,
    pub author: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentQueryRoot {
    pub comment: Comment,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteQueryRoot {
    pub note: Note,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraphQLQueryResponse<T> {
    Err { errors: Vec<GraphQLError> },
    Ok { data: T },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphQLQueryRequest {
    pub query: String,
    pub variables: serde_json::Value,
}

// TODO: めっちゃ適当
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
