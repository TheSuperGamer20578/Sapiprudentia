pub(super) mod user;
pub(super) mod session;
pub(super) mod subject;
pub(super) mod todo;
pub(super) mod note;
mod assessment;

use async_graphql::{Context, Result, Object};
use rocket::http::Status;
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::assessment::Assessment;
use crate::api::graphql::query::note::Note;
use crate::api::graphql::query::subject::Subject;
use crate::api::graphql::query::todo::Todo;
use crate::auth::User;

#[allow(clippy::module_name_repetitions)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get authenticated user data or null if not authenticated.
    async fn current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        Ok(ctx.data::<Option<User>>()?.clone())
    }

    /// Get list of all notes owned by the authenticated user.
    /// Requires authentication.
    async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        Ok(query_as!(Note, /* language=postgresql */ "SELECT * FROM notes WHERE owner = $1;", user.id)
            .fetch_all(pool).await?)
    }

    /// Get a single note by ID.
    /// Requires authentication.
    async fn note(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the note to get.")] id: i32) -> Result<Note> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        query_as!(Note, /* language=postgresql */ "SELECT * FROM notes WHERE owner = $1 AND id = $2 LIMIT 1;", user.id, id)
            .fetch_optional(pool).await?.ok_or(Status::NotFound.into())
    }

    /// Get list of all subjects owned by the authenticated user.
    /// Requires authentication.
    async fn subjects(&self, ctx: &Context<'_>) -> Result<Vec<Subject>> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        Ok(query_as!(Subject, /* language=postgresql */ "SELECT * FROM subjects WHERE owner = $1;", user.id)
            .fetch_all(pool).await?)
    }

    /// Get a single subject by ID.
    /// Requires authentication.
    async fn subject(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the subject to get.")] id: i32) -> Result<Subject> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        query_as!(Subject, /* language=postgresql */ "SELECT * FROM subjects WHERE owner = $1 AND id = $2 LIMIT 1;", user.id, id)
            .fetch_optional(pool).await?.ok_or(Status::NotFound.into())
    }

    /// Get list of all todos owned by the authenticated user.
    /// Requires authentication.
    async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<Todo>> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        Ok(query_as!(Todo, /* language=postgresql */ "SELECT * FROM todos WHERE owner = $1;", user.id)
            .fetch_all(pool).await?)
    }

    /// Get a single to-do by ID.
    /// Requires authentication.
    async fn todo(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the to-do to get.")] id: i32) -> Result<Todo> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        query_as!(Todo, /* language=postgresql */ "SELECT * FROM todos WHERE owner = $1 AND id = $2 LIMIT 1;", user.id, id)
            .fetch_optional(pool).await?.ok_or(Status::NotFound.into())
    }

    async fn assessments(&self, ctx: &Context<'_>) -> Result<Vec<Assessment>> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        Ok(query_as!(Assessment, /* language=postgresql */ "SELECT * FROM assessments WHERE owner = $1;", user.id)
            .fetch_all(pool).await?)
    }

    async fn assessment(&self, ctx: &Context<'_>, id: i32) -> Result<Assessment> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        query_as!(Assessment, /* language=postgresql */ "SELECT * FROM assessments WHERE owner = $1 AND id = $2 LIMIT 1;", user.id, id)
            .fetch_optional(pool).await?.ok_or(Status::NotFound.into())
    }
}
