#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]

mod auth;
mod api;

use rocket::Config;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};
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
        .mount("/auth", &**auth::ROUTES)
        .manage(db.clone())
        .attach(Cors::from_options(&CorsOptions::default()
            .allowed_origins(AllowedOrigins::some_exact(&secrets.get("CORS_ALLOWED_ORIGINS").unwrap_or_default().split(' ').collect::<Vec<_>>()))
            .allow_credentials(true)
        ).unwrap());
    #[cfg(feature = "api_graphql")] {
        rocket = rocket
            .manage(create_schema(db))
            .mount("/graphql", &**api::graphql::ROUTES);
    }
    Ok(rocket.into())
}
