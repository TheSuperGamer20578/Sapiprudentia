use async_graphql::{Object, SimpleObject};
use chrono::NaiveDateTime;

#[derive(SimpleObject)]
pub struct Document {
    /// The ID of the document.
    pub id: i32,

    /// The date and time the document was created.
    pub created_at: NaiveDateTime,

    /// The title of the document.
    pub title: String,

    /// The content of the document as output and understood by TipTap.
    pub content: Option<serde_json::Value>,

    /// The date and time the document was last modified.
    pub last_modified: NaiveDateTime,

    #[graphql(skip)]
    pub owner: i32,
}
