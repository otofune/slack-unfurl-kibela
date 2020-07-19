use serde::{Deserialize, Serialize};

type ID = String;

#[derive(Debug, Serialize, Deserialize)]
struct UserAvatarImage {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    account: String,
    id: ID,
    #[serde(rename = "avatarImage")]
    avatar_image: UserAvatarImage,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: ID,
    title: String,
    url: String,
    #[serde(rename = "publishedAt")]
    published_at: String,
    summary: String,
    author: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct Comment {
    id: ID,
    #[serde(rename = "publishedAt")]
    published_at: String,
    summary: String,
    author: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentQueryRoot {
    comment: Comment,
}
#[derive(Debug, Serialize, Deserialize)]
struct NoteQueryRoot {
    note: Note,
}

// TODO: このへんジェネリクスでなんとかならないか?
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentQuerySuccessfulResponse {
    data: CommentQueryRoot,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteQuerySuccessfulResponse {
    data: NoteQueryRoot,
}
