mod document;
mod session;
mod subject;
mod todo;
mod user;

use async_graphql::{Context, Object, Result};
use chrono::NaiveDate;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use sqlx::{PgPool, query, query_as};
use crate::api::graphql::mutation::document::DocumentMutation;
use crate::api::graphql::mutation::subject::SubjectMutation;
use crate::api::graphql::mutation::todo::TodoMutation;
use crate::api::graphql::mutation::user::UserMutation;
use crate::api::graphql::query::document::Document;
use crate::api::graphql::query::subject::Subject;
use crate::api::graphql::query::todo::Todo;
use crate::auth::User;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Get a document for modification.
    /// Requires authentication.
    async fn document(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the document to modify.")] id: i32) -> Result<DocumentMutation> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query(/* language=postgresql */ "SELECT 1 FROM documents WHERE owner = $1 AND id = $2 LIMIT 1;")
            .bind(user.id)
            .bind(id)
            .fetch_optional(ctx.data::<PgPool>()?).await?.ok_or(Status::NotFound)?;
        Ok(DocumentMutation(id))
    }

    /// Creates a new document. Returns the newly created document.
    /// Requires authentication.
    /// Deprecated.
    #[graphql(deprecation = "For development purposes only, will be removed in the future in favour of automatic creation.")]
    async fn create_document(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = r#"The title of the document. Default: """#, default = "", validator(max_length = 255))] title: String,
        #[graphql(desc = "The content of the document. Default: null", default)] content: Option<serde_json::Value>,
    ) -> Result<Document> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query_as!(Document, /* language=postgresql */ "INSERT INTO documents (owner, title, content) VALUES ($1, $2, $3) RETURNING *;", user.id, title, content)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }

    /// Get a subject for modification.
    /// Requires authentication.
    async fn subject(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the subject to modify.")] id: i32) -> Result<SubjectMutation> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query(/* language=postgresql */ "SELECT 1 FROM subjects WHERE owner = $1 AND id = $2 LIMIT 1;")
            .bind(user.id)
            .bind(id)
            .fetch_optional(ctx.data::<PgPool>()?).await?.ok_or(Status::NotFound)?;
        Ok(SubjectMutation(id))
    }

    /// Creates a new subject. Returns the newly created subject.
    /// Requires authentication.
    async fn create_subject(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The name of the subject.", validator(max_length = 255))] name: String,
        #[graphql(desc = "The class of the subject.", validator(max_length = 16))] class: String,
        #[graphql(desc = "Whether the subject is active. Default: true", default = true)] active: bool,
        #[graphql(desc = "The subject's Google Classroom ID. Default: null", default, validator(max_length = 16))] google_classroom_id: Option<String>,
    ) -> Result<Subject> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query_as!(Subject, /* language=postgresql */ "INSERT INTO subjects (owner, name, class, active, google_classroom_id) VALUES ($1, $2, $3, $4, $5) RETURNING *;", user.id, name, class, active, google_classroom_id)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }

    /// Get a to-do for modification.
    /// Requires authentication.
    async fn todo(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the to-do to modify.")] id: i32) -> Result<TodoMutation> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query(/* language=postgresql */ "SELECT 1 FROM todos WHERE owner = $1 AND id = $2 LIMIT 1;")
            .bind(user.id)
            .bind(id)
            .fetch_optional(ctx.data::<PgPool>()?).await?.ok_or(Status::NotFound)?;
        Ok(TodoMutation(id))
    }

    /// Creates a new to-do. Returns the newly created to-do.
    /// Requires authentication.
    async fn create_todo(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The title of the to-do.", validator(max_length = 255))] title: String,
        #[graphql(desc = "The completed status of the to-do. Default: false", default = false)] completed: bool,
        #[graphql(desc = "The due date of the to-do. Default: null", default)] due: Option<NaiveDate>,
        #[graphql(desc = "The archived status of the to-do. Default: false", default = false)] archived: bool,
        #[graphql(desc = "The to-do's subject's ID. Default: null", default)] subject: Option<i32>,
    ) -> Result<Todo> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query_as!(Todo, /* language=postgresql */ "INSERT INTO todos (owner, title, completed, due, archived, subject) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;", user.id, title, completed, due, archived, subject)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }

    /// Get the current user for modification.
    /// Requires authentication.
    async fn current_user(&self, ctx: &Context<'_>) -> Result<UserMutation> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        Ok(UserMutation(user.id))
    }
}
