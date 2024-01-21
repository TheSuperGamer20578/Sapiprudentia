use async_graphql::{Context, Object, Result};
use sqlx::{PgPool, query, query_as};
use crate::api::graphql::query::document::Document;

pub struct DocumentMutation(pub i32);

#[Object]
impl DocumentMutation {
    /// Delete the document. Always returns true or an error.
    async fn delete(&self, ctx: &Context<'_>) -> Result<bool> {
        query!(/* language=postgresql */ "DELETE FROM documents WHERE id = $1;", self.0)
            .execute(ctx.data::<PgPool>()?).await?;
        Ok(true)
    }
    
    /// Updates the document's title. Returns the updated document.
    async fn title(&self, ctx: &Context<'_>, #[graphql(desc = "The new title of the document.", validator(max_length = 255))] title: String) -> Result<Document> {
        query_as!(Document, /* language=postgresql */ "UPDATE documents SET title = $2 WHERE id = $1 RETURNING *;", self.0, title)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the document's content. Returns the updated document.
    async fn content(&self, ctx: &Context<'_>, #[graphql(desc = "The new content of the document.")] content: Option<serde_json::Value>) -> Result<Document> {
        query_as!(Document, /* language=postgresql */ "UPDATE documents SET content = $2 WHERE id = $1 RETURNING *;", self.0, content)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
}
