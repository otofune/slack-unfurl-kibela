use chrono::prelude::{DateTime, Utc};
use serde::Deserialize;

pub type ID = String;

#[derive(Debug, Deserialize)]
pub struct UserAvatarImage {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub account: String,
    pub id: ID,
    #[serde(rename = "avatarImage")]
    pub avatar_image: UserAvatarImage,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    pub id: ID,
    pub title: String,
    pub url: String,
    #[serde(rename = "publishedAt")]
    pub published_at: DateTime<Utc>,
    pub summary: String,
    pub author: User,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: ID,
    #[serde(rename = "publishedAt")]
    pub published_at: DateTime<Utc>,
    pub summary: String,
    pub author: User,
}

#[derive(Debug, Deserialize)]
pub struct CommentQueryRoot {
    pub comment: Comment,
}
#[derive(Debug, Deserialize)]
pub struct NoteQueryRoot {
    pub note: Note,
}
