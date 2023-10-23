#![warn(clippy::pedantic)]

mod api;
mod documents;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_rds::Postgres] db: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!().run(&db).await.unwrap();

    Ok(rocket::build()
        .mount("/static", FileServer::from(relative!("/static")).rank(10))
        .mount("/static", FileServer::from(relative!("/dist")).rank(11))
        .mount("/api", &**api::ROUTES)
        .mount("/document", &**documents::ROUTES)
        .manage(db)
        .attach(Template::fairing())
        .into())
}
