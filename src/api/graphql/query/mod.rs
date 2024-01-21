pub(super) mod document;
pub(super) mod user;
pub(super) mod session;
pub(super) mod subject;
pub(super) mod todo;

use async_graphql::{Context, Result, Object};
use rocket::http::Status;
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::document::Document;
use crate::api::graphql::query::subject::Subject;
use crate::api::graphql::query::todo::Todo;
use crate::auth::User;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Get authenticated user data or null if not authenticated.
    async fn current_user(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        Ok(ctx.data::<Option<User>>()?.clone())
    }

    /// Get list of all documents owned by the authenticated user.
    /// Requires authentication.
    async fn documents(&self, ctx: &Context<'_>) -> Result<Vec<Document>> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        Ok(query_as!(Document, /* language=postgresql */ "SELECT * FROM documents WHERE owner = $1;", user.id)
            .fetch_all(pool).await?)
    }

    /// Get a single document by ID.
    /// Requires authentication.
    async fn document(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the document to get.")] id: i32) -> Result<Document> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        let pool = ctx.data::<PgPool>()?;
        query_as!(Document, /* language=postgresql */ "SELECT * FROM documents WHERE owner = $1 AND id = $2 LIMIT 1;", user.id, id)
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
}
