#![allow(clippy::unused_async)]
#![allow(clippy::no_effect_underscore_binding)]

mod query;
mod mutation;

use async_graphql::EmptySubscription;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use lazy_static::lazy_static;
use rocket::{get, post, Route, routes, State};
use sqlx::PgPool;
use crate::auth::User;

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        schema,
        post,
    ];
}

pub type Schema = async_graphql::Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;

pub fn create_schema(pool: PgPool) -> Schema {
    Schema::build(query::QueryRoot, mutation::MutationRoot, EmptySubscription)
        .data(pool)
        .finish()
}

#[get("/")]
fn schema(schema: &State<Schema>) -> String {
    schema.sdl()
}

#[post("/", data = "<request>", format = "application/json")]
async fn post<'a>(schema: &State<Schema>, user: Option<User>, request: GraphQLRequest) -> GraphQLResponse {
    request
        .data(user)
        .execute(&**schema).await
}
