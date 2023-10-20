#![warn(clippy::pedantic)]

use rocket::{get, routes};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{context, Template};
use sqlx::PgPool;

#[get("/")]
fn index() -> Template {
    Template::render("template", context! {})
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_rds::Postgres] db: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!().run(&db).await.unwrap();

    #[allow(clippy::no_effect_underscore_binding)]
    Ok(rocket::build()
        .mount("/static", FileServer::from(relative!("static")).rank(10))
        .mount("/static", FileServer::from(concat!(env!("OUT_DIR"), "/js")).rank(11))
        .mount("/static", FileServer::from(concat!(env!("OUT_DIR"), "/css")).rank(12))
        .mount("/", routes![
            index,
        ])
        .manage(db)
        .attach(Template::fairing())
        .into())
}
