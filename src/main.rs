#![warn(clippy::pedantic)]

mod api;
mod documents;
mod index;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;
use sqlx::PgPool;

async fn global_context(db: &PgPool) -> impl Serialize {
    let documents: Vec<_> = sqlx::query!(/* language=sql */ "SELECT id, title FROM documents;")
        .fetch_all(db).await.unwrap()
        .into_iter()
        .map(|document| context! {
            id: document.id,
            title: if document.title.is_empty() {"Untitled".into()} else {document.title},
        })
        .collect();
    context! {
        documents: documents,
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_rds::Postgres] db: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!().run(&db).await.unwrap();

    Ok(rocket::build()
        .mount("/static", FileServer::from(relative!("/static")).rank(10))
        .mount("/static", FileServer::from(relative!("/dist")).rank(11))
        .mount("/api", &**api::ROUTES)
        .mount("/", &**index::ROUTES)
        .mount("/document", &**documents::ROUTES)
        .manage(db)
        .attach(Template::fairing())
        .into())
}
