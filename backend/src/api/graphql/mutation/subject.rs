use async_graphql::{Context, Object, Result};
use sqlx::{query, query_as};
use crate::api::graphql::query::subject::Subject;

pub struct SubjectMutation(pub i32);

#[Object]
impl SubjectMutation {
    /// Delete the subject. Always returns true or an error.
    async fn delete(&self, ctx: &Context<'_>) -> Result<bool> {
        query!(/* language=postgresql */ "DELETE FROM subjects WHERE id = $1;", self.0)
            .execute(ctx.data::<sqlx::PgPool>()?).await?;
        Ok(true)
    }
    
    /// Updates the subject's name. Returns the updated subject.
    async fn name(&self, ctx: &Context<'_>, #[graphql(desc = "The new name of the subject.", validator(max_length = 255))] name: String) -> Result<Subject> {
        query_as!(Subject, /* language=postgresql */ "UPDATE subjects SET name = $2 WHERE id = $1 RETURNING *;", self.0, name)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the subject's class. Returns the updated subject.
    async fn class(&self, ctx: &Context<'_>, #[graphql(desc = "The new class of the subject.", validator(max_length = 16))] class: Option<String>) -> Result<Subject> {
        query_as!(Subject, /* language=postgresql */ "UPDATE subjects SET class = $2 WHERE id = $1 RETURNING *;", self.0, class)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the active status of the subject. Returns the updated subject.
    /// Active subjects are shown in the navigation.
    async fn active(&self, ctx: &Context<'_>, #[graphql(desc = "The new active status of the subject.")] active: bool) -> Result<Subject> {
        query_as!(Subject, /* language=postgresql */ "UPDATE subjects SET active = $2 WHERE id = $1 RETURNING *;", self.0, active)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the subject's Google Classroom ID. Returns the updated subject.
    async fn google_classroom_id(&self, ctx: &Context<'_>, #[graphql(desc = "The new Google Classroom ID of the subject.", validator(max_length = 16))] google_classroom_id: Option<String>) -> Result<Subject> {
        query_as!(Subject, /* language=postgresql */ "UPDATE subjects SET google_classroom_id = $2 WHERE id = $1 RETURNING *;", self.0, google_classroom_id)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }
}
