use lazy_static::lazy_static;
use rocket::{get, Route, routes, State};
use rocket_dyn_templates::{context, Template};
use sqlx::PgPool;
use crate::global_context;

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        index,
        react,
    ];
}

#[get("/")]
async fn index(db: &State<PgPool>) -> Template {
    Template::render("index", context! {
        global: global_context(db).await,
    })
}

#[get("/react")]
async fn react() -> Template {
    Template::render("react", context! {})
}