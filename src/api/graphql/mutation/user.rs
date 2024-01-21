use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use async_graphql::{Context, Object, Result};
use rocket::http::Status;
use sqlx::{PgPool, query, query_as};
use crate::api::graphql::mutation::session::SessionMutation;
use crate::auth::{ARGON2, User};

pub struct UserMutation(pub i32);

#[Object]
impl UserMutation {
    /// Delete the user. Always returns true or an error.
    async fn delete(&self, ctx: &Context<'_>) -> Result<bool> {
        query!(/* language=postgresql */ "DELETE FROM users WHERE id = $1;", self.0)
            .execute(ctx.data::<PgPool>()?).await?;
        Ok(true)
    }

    /// Updates the user's username. Returns the updated user.
    async fn username(&self, ctx: &Context<'_>, #[graphql(desc = "The new username of the user.", validator(max_length = 16))] username: String) -> Result<User> {
        query_as!(User, /* language=postgresql */ "UPDATE users SET username = $2 WHERE id = $1 RETURNING id, username, name, created_at, require_password_change, account_type, email;", self.0, username)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the user's name. Returns the updated user.
    async fn name(&self, ctx: &Context<'_>, #[graphql(desc = "The new name of the user.", validator(max_length = 255))] name: String) -> Result<User> {
        query_as!(User, /* language=postgresql */ "UPDATE users SET name = $2 WHERE id = $1 RETURNING id, username, name, created_at, require_password_change, account_type, email;", self.0, name)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the user's password. Returns the updated user.
    async fn password(&self, ctx: &Context<'_>, #[graphql(desc = "The new password of the user.")] password: String) -> Result<User> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = ARGON2.hash_password(password.as_bytes(), &salt)?.to_string();
        query_as!(User, /* language=postgresql */ "UPDATE users SET password = $2, require_password_change = FALSE WHERE id = $1 RETURNING id, username, name, created_at, require_password_change, account_type, email;", self.0, hash)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
    
    /// Updates the user's email. Returns the updated user.
    async fn email(&self, ctx: &Context<'_>, #[graphql(desc = "The new email of the user.", validator(email, max_length = 255))] email: String) -> Result<User> {
        query_as!(User, /* language=postgresql */ "UPDATE users SET email = $2 WHERE id = $1 RETURNING id, username, name, created_at, require_password_change, account_type, email;", self.0, email)
            .fetch_one(ctx.data::<PgPool>()?).await.map_err(Into::into)
    }
    
    /// Get a session for modification.
    async fn session(&self, ctx: &Context<'_>, #[graphql(desc = "The ID of the session to modify.")] id: i32) -> Result<SessionMutation> {
        query(/* language=postgresql */ "SELECT 1 FROM sessions WHERE user_id = $1 AND id = $2 LIMIT 1;")
            .bind(self.0)
            .bind(id)
            .fetch_optional(ctx.data::<PgPool>()?).await?.ok_or(Status::NotFound)?;
        Ok(SessionMutation(id))
    }
}
