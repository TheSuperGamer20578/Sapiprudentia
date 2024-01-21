use async_graphql::{Context, Object, Result};
use chrono::NaiveDate;
use rocket::http::Status;
use sqlx::query_as;
use crate::api::graphql::query::todo::Todo;
use crate::auth::User;

pub struct TodoMutation(pub i32);

#[Object]
impl TodoMutation {
    /// Delete the to-do. Returns the deleted to-do.
    /// Also deletes children.
    async fn delete(&self, ctx: &Context<'_>) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "DELETE FROM todos WHERE id = $1 RETURNING *;", self.0)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the to-do's title. Returns the updated to-do.
    async fn title(&self, ctx: &Context<'_>, #[graphql(desc = "The new title.", validator(max_length = 255))] title: String) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "UPDATE todos SET title = $2 WHERE id = $1 RETURNING *;", self.0, title)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the to-do's completed status. Returns the updated to-do.
    async fn completed(&self, ctx: &Context<'_>, #[graphql(desc = "The new completed status.")] completed: bool) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "UPDATE todos SET completed = $2 WHERE id = $1 RETURNING *;", self.0, completed)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the to-do's subject. Returns the updated to-do.
    async fn subject(&self, ctx: &Context<'_>, #[graphql(desc = "The new subject's ID.")] id: Option<i32>) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "UPDATE todos SET subject = $2 WHERE id = $1 RETURNING *;", self.0, id)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Changes the to-do's parent. Returns the updated to-do.
    async fn parent(&self, ctx: &Context<'_>, #[graphql(desc = "The new parent's ID.")] id: Option<i32>) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "UPDATE todos SET parent = $2 WHERE id = $1 RETURNING *;", self.0, id)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the to-do's due date. Returns the updated to-do.
    async fn due(&self, ctx: &Context<'_>, #[graphql(desc = "The new due date.")] due: Option<NaiveDate>) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "UPDATE todos SET due = $2 WHERE id = $1 RETURNING *;", self.0, due)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Updates the to-do's archived status. Returns the updated to-do.
    /// Archived to-dos are hidden from the default view.
    /// Does nothing if the to-do is a child.
    async fn archived(&self, ctx: &Context<'_>, #[graphql(desc = "The new archived status.")] archived: bool) -> Result<Todo> {
        query_as!(Todo, /* language=postgresql */ "UPDATE todos SET archived = $2 WHERE id = $1 RETURNING *;", self.0, archived)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }

    /// Creates a child to-do. Returns the newly created to-do.
    /// Requires authentication.
    async fn create_child(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "The title of the to-do.", validator(max_length = 255))] title: String,
        #[graphql(desc = "The completed status of the to-do. Default: false", default = false)] completed: bool,
        #[graphql(desc = "The due date of the to-do. Default: null", default)] due: Option<NaiveDate>,
    ) -> Result<Todo> {
        let Some(user) = ctx.data::<Option<User>>()? else {
            return Err(Status::Unauthorized.into());
        };
        query_as!(Todo, /* language=postgresql */ "INSERT INTO todos (owner, title, completed, parent, due) VALUES ($1, $2, $3, $4, $5) RETURNING *;", user.id, title, completed, self.0, due)
            .fetch_one(ctx.data::<sqlx::PgPool>()?).await.map_err(Into::into)
    }
}
