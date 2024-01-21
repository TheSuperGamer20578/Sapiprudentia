#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]

mod frontend;
mod auth;
mod api;

#[cfg(not(debug_assertions))]
use rocket::fs::{FileServer, relative};
#[cfg(not(debug_assertions))]
use rocket::routes;
use rocket::Config;
use rocket_dyn_templates::Template;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use crate::api::graphql::create_schema;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    sqlx::migrate!().run(&db).await.unwrap();

    #[allow(unused_mut)]
    let mut rocket = rocket::build()
        .configure(Config::figment()
            .merge(("secret_key", secrets.get("SECRET_KEY").unwrap()))
        )
        .mount("/", &**frontend::ROUTES)
        .mount("/api/auth", &**auth::ROUTES)
        .manage(db.clone())
        .attach(Template::fairing());
    #[cfg(feature = "api_v1")] {
        rocket = rocket
            .mount("/api/v1", &**api::v1::ROUTES);
    }
    #[cfg(feature = "api_graphql")] {
        rocket = rocket
            .manage(create_schema(db))
            .mount("/api/graphql", &**api::graphql::ROUTES);
    }
    #[cfg(not(debug_assertions))] {
        rocket = rocket
            .mount("/static", FileServer::from(relative!("dist")))
            .mount("/", routes![frontend::serviceworker]);
    }
    Ok(rocket.into())
}
