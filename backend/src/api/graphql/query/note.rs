use async_graphql::{ComplexObject, Context, SimpleObject, Result};
use chrono::NaiveDate;
use rocket::http::Status;
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::subject::Subject;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Note {
    /// The note’s ID
    pub id: i32,
    #[graphql(skip)]
    pub owner: i32,
    /// The date of the note’s class
    pub date: NaiveDate,
    /// The note’s title
    pub title: String,
    #[graphql(skip)]
    pub subject: Option<i32>,
    /// The document’s content as understood by TipTap
    pub content: serde_json::Value,
}

#[ComplexObject]
impl Note {
    /// The subject of the note.
    async fn subject(&self, ctx: &Context<'_>) -> Result<Option<Subject>> {
        Ok(if let Some(subject) = self.subject {
            Some(query_as!(Subject, /* language=postgresql */ "SELECT * FROM subjects WHERE id = $1 LIMIT 1;", subject)
                .fetch_one(ctx.data::<PgPool>()?).await.or(Err(Status::InternalServerError))?)
        } else {
            None
        })
    }
}
