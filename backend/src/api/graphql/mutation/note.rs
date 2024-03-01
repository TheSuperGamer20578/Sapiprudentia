use async_graphql::{Context, Object, Result};
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::note::Note;

pub struct NoteMutation(pub i32);

#[Object]
impl NoteMutation {
    /// Delete the note. Returns the deleted note.
    async fn delete(&self, ctx: &Context<'_>) -> Result<Note> {
        query_as!(Note, /* language=postgresql */ "DELETE FROM notes WHERE id = $1 RETURNING *;", self.0)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the note’s content. Returns the updated note.
    async fn content(&self, ctx: &Context<'_>, #[graphql(desc = "The new content of the note.")] content: Option<serde_json::Value>) -> Result<Note> {
        query_as!(Note, /* language=postgresql */ "UPDATE notes SET content = $2 WHERE id = $1 RETURNING *;", self.0, content)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the note's date. Returns the updated note.
    async fn date(&self, ctx: &Context<'_>, #[graphql(desc = "The new date of the note.")] date: Option<chrono::NaiveDate>) -> Result<Note> {
        query_as!(Note, /* language=postgresql */ "UPDATE notes SET date = $2 WHERE id = $1 RETURNING *;", self.0, date)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the note's title. Returns the updated note.
    async fn title(&self, ctx: &Context<'_>, #[graphql(desc = "The new title of the note.")] title: String) -> Result<Note> {
        query_as!(Note, /* language=postgresql */ "UPDATE notes SET title = $2 WHERE id = $1 RETURNING *;", self.0, title)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the note's subject. Returns the updated note.
    async fn subject(&self, ctx: &Context<'_>, #[graphql(desc = "The new subject of the note.")] subject: Option<i32>) -> Result<Note> {
        query_as!(Note, /* language=postgresql */ "UPDATE notes SET subject = $2 WHERE id = $1 RETURNING *;", self.0, subject)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
}
