use async_graphql::{Context, Object, Result};
use chrono::NaiveDateTime;
use rocket::http::Status;
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::session::Session;
use crate::auth::User;

#[Object]
impl User {
    /// The user ID.
    async fn id(&self) -> i32 {
        self.id
    }

    /// The unique username used for login.
    async fn username(&self) -> &String {
        &self.username
    }

    /// The name used in the UI.
    async fn name(&self) -> &String {
        &self.name
    }

    /// The email address of the user.
    async fn email(&self) -> &String {
        &self.email
    }

    // TODO: account_type

    /// The date and time the user was created.
    async fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    /// Weather the user is required to change their password on next login.
    async fn require_password_change(&self) -> bool {
        self.require_password_change
    }

    /// List of all active sessions for the user.
    async fn sessions(&self, ctx: &Context<'_>) -> Result<Vec<Session>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(query_as!(Session, /* language=postgresql */ "SELECT * FROM sessions WHERE user_id = $1;", self.id)
            .fetch_all(pool).await?)
    }

    /// Get a single session by ID.
    async fn session(&self, ctx: &Context<'_>, id: i32) -> Result<Session> {
        let pool = ctx.data::<PgPool>()?;
        query_as!(Session, /* language=postgresql */ "SELECT * FROM sessions WHERE user_id = $1 AND id = $2 LIMIT 1;", self.id, id)
            .fetch_optional(pool).await?.ok_or(Status::NotFound.into())
    }
}
