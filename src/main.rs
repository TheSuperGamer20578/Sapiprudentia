#![warn(clippy::pedantic)]

mod api_v1;
mod frontend;
mod auth;

use rocket::fs::{FileServer, relative};
use rocket::{Config, routes};
use rocket_dyn_templates::Template;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!().run(&db).await.unwrap();

    let mut rocket = rocket::build()
        .configure(Config::figment()
            .merge(("secret_key", secrets.get("SECRET_KEY").unwrap()))
        )
        .mount("/api/v1", &**api_v1::ROUTES)
        .mount("/", &**frontend::ROUTES)
        .manage(db)
        .attach(Template::fairing());
    #[cfg(not(skip_webpack))] {
        rocket = rocket
            .mount("/static", FileServer::from(relative!("dist")))
            .mount("/", routes![frontend::serviceworker]);
    }
    Ok(rocket.into())
}
