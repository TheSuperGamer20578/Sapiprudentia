use async_graphql::{Context, Object, Result};
use sqlx::query;

pub struct SessionMutation(pub i32);

#[Object]
impl SessionMutation {
    /// Revoke the session. Always returns true or an error.
    async fn delete(&self, ctx: &Context<'_>) -> Result<bool> {
        query!(/* language=postgresql */ "DELETE FROM sessions WHERE id = $1;", self.0)
            .execute(ctx.data::<sqlx::PgPool>()?).await?;
        Ok(true)
    }
}
