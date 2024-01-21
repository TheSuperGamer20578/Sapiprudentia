use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use chrono::NaiveDate;
use rocket::http::Status;
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::subject::Subject;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Todo {
    /// The ID of the to-do.
    pub id: i32,

    #[graphql(skip)]
    pub owner: i32,

    /// The title of the to-do.
    pub title: String,

    /// Whether the to-do is completed.
    pub completed: bool,

    #[graphql(skip)]
    pub subject: Option<i32>,

    #[graphql(skip)]
    pub parent: Option<i32>,

    /// The due date of the to-do.
    pub due: Option<NaiveDate>,

    /// Whether the to-do is archived.
    /// Archived todos are hidden from the default view.
    pub archived: bool,
}

#[ComplexObject]
impl Todo {
    /// The subject of the to-do.
    async fn subject(&self, ctx: &Context<'_>) -> Result<Option<Subject>> {
        Ok(if let Some(subject) = self.subject {
            Some(query_as!(Subject, /* language=postgresql */ "SELECT * FROM subjects WHERE id = $1 LIMIT 1;", subject)
                .fetch_one(ctx.data::<PgPool>()?).await.or(Err(Status::InternalServerError))?)
        } else {
            None
        })
    }

    /// The parent of the to-do.
    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<Todo>> {
        Ok(if let Some(parent) = self.parent {
            Some(query_as!(Todo, /* language=postgresql */ "SELECT * FROM todos WHERE id = $1 LIMIT 1;", parent)
                .fetch_one(ctx.data::<PgPool>()?).await.or(Err(Status::InternalServerError))?)
        } else {
            None
        })
    }
    
    /// The children of the to-do.
    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<Todo>> {
        query_as!(Todo, /* language=postgresql */ "SELECT * FROM todos WHERE parent = $1;", self.id)
            .fetch_all(ctx.data::<PgPool>()?).await.or(Err(Status::InternalServerError.into()))
    }
}
