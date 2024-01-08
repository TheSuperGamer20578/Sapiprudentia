#![warn(clippy::pedantic)]

mod api_v1;
mod frontend;
mod auth;

use rocket::fs::FileServer;
use rocket::routes;
use rocket_dyn_templates::Template;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_aws_rds::Postgres] db: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!().run(&db).await.unwrap();

    let mut rocket = rocket::build()
        .mount("/api/v1", &**api_v1::ROUTES)
        .mount("/", &**frontend::ROUTES)
        .manage(db)
        .attach(Template::fairing());
    #[cfg(not(skip_webpack))] {
        rocket = rocket
            .mount("/static", FileServer::from(env!("OUT_DIR")))
            .mount("/", routes![frontend::serviceworker]);
    }
    Ok(rocket.into())
}
