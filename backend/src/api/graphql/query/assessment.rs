use std::mem;
use async_graphql::{ComplexObject, Context, SimpleObject, Result, Enum};
use chrono::NaiveDate;
use rocket::http::Status;
use sqlx::{PgPool, query_as};
use crate::api::graphql::query::subject::Subject;

#[derive(Enum, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum AssessmentStatus {
    NotIssued = 0,
    NotStarted = 1,
    InProgress = 2,
    Finished = 3,
    ResultsReceived = 4,
}

impl TryFrom<u8> for AssessmentStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=4).contains(&value) {
            return Err(());
        }
        unsafe {
            Ok(mem::transmute(value))
        }
    }
}

impl From<i16> for AssessmentStatus {
    fn from(value: i16) -> Self {
        u8::try_from(value).unwrap().try_into().unwrap()
    }
}

#[derive(Enum, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum DuePeriod {
    BS = 0,
    RC = 1,
    One = 2,
    Two = 3,
    Three = 4,
    Four = 5,
    AS = 6,
    Midnight = 7,
}

impl TryFrom<u8> for DuePeriod {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=7).contains(&value) {
            return Err(());
        }
        unsafe {
            Ok(mem::transmute(value))
        }
    }
}

impl From<i16> for DuePeriod {
    fn from(value: i16) -> Self {
        u8::try_from(value).unwrap().try_into().unwrap()
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Assessment {
    pub id: i32,
    #[graphql(skip)]
    pub owner: i32,
    #[graphql(skip)]
    pub subject: i32,
    pub title: String,
    pub exam: bool,
    pub status: AssessmentStatus,
    pub weight: i16,
    pub due: NaiveDate,
    pub due_period: DuePeriod,
    pub issued: Option<NaiveDate>,
    pub mark_out_of: Option<i16>,
    pub mark: Option<i16>,
    pub notification: Option<String>,
    pub submission: Option<String>,
    pub reference: Option<String>,

}

#[ComplexObject]
impl Assessment {
    async fn subject(&self, ctx: &Context<'_>) -> Result<Subject> {
        query_as!(Subject, /* language=postgresql */ "SELECT * FROM subjects WHERE id = $1 LIMIT 1;", self.subject)
            .fetch_one(ctx.data::<PgPool>()?).await.or(Err(Status::InternalServerError)).map_err(Into::into)
    }
}
