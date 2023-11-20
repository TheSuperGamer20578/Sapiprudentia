use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use email_address::EmailAddress;
use lazy_static::lazy_static;
use rocket::{delete, get, patch, post, Route, routes, State};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use serde::{Deserialize};
use sqlx::{PgPool, query};
use crate::auth::{ARGON2, Identity, User};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        document::get,
        document::update,
        login,
        current_user,
        logout,
        update_current_user,
    ];
}

macro_rules! crud {
    ($module:ident, $route:literal, $route_id:literal, $table:literal, {$($field:ident: $field_type:ty),*$(,)?}) => {
        #[allow(dead_code)]  // rocket emitts structs
        #[allow(unused_imports)]  // and imports
        mod $module {
            use rocket::http::Status;
            use rocket::serde::json::Json;
            use sqlx::{FromRow, PgPool, query, query_as};
            use rocket::{delete, get, patch, post, State};
            use serde::{Serialize, Deserialize};
            use itertools::Itertools;
            use crate::auth::User;

            #[derive(FromRow)]
            pub(super) struct Full {
                id: i32,
                owner: i32,
                $($field: $field_type),*
            }

            #[derive(Serialize, FromRow)]
            pub(super) struct Send {
                id: i32,
                $($field: $field_type),*
            }

            impl From<Full> for Send {
                fn from(value: Full) -> Self {
                    Self {
                        id: value.id,
                        $($field: value.$field),*
                    }
                }
            }

            #[derive(Deserialize)]
            pub(super) struct Create {
                $($field: $field_type),*
            }

            #[derive(Deserialize)]
            pub(super) struct Update {
                $($field: Option<$field_type>),*
            }

            #[get($route)]
            pub(super) async fn list(db: &State<PgPool>, user: User) -> Json<Vec<Send>> {
                Json(query_as(&format!("SELECT * FROM {} WHERE owner = $1;", $table))
                    .bind(user.id)
                    .fetch_all(&**db).await.unwrap())
            }

            #[post($route, data = "<create>")]
            pub(super) async fn create(create: Json<Create>, db: &State<PgPool>, user: User) -> Status {
                query(&format!("INSERT INTO {} (owner, {}) VALUES ($1, {});", $table, stringify!($($field),*), vec! [$(stringify!($field)),*].iter().enumerate().map(|(i, _)| format!("${}", i + 2)).join(", ")))
                    .bind(user.id)
                    $(.bind(&create.$field))*
                    .execute(&**db).await.unwrap();
                Status::NoContent
            }

            #[get($route_id)]
            pub(super) async fn get(id: i32, db: &State<PgPool>, user: User) -> Option<Result<Json<Send>, Status>> {
                let object: Full = query_as(&format!("SELECT * FROM {} WHERE id = $1;", $table))
                    .bind(id)
                    .fetch_optional(&**db).await.unwrap()?;
                if object.owner != user.id {
                    return Some(Err(Status::Forbidden));
                }
                Some(Ok(Json(object.into())))
            }

            #[patch($route_id, data = "<update>")]
            pub(super) async fn update(update: Json<Update>, id: i32, db: &State<PgPool>, user: User) -> Option<Status> {
                let (owner,): (i32,) = query_as(&format!("SELECT owner FROM {} WHERE id = $1;", $table))
                    .bind(id)
                    .fetch_optional(&**db).await.unwrap()?;
                if owner != user.id {
                    return Some(Status::Forbidden);
                }

                $(
                    if let Some(value) = &update.$field {
                        query(&format!("UPDATE {} SET {} = $2 WHERE id = $1;", $table, stringify!($field)))
                            .bind(id)
                            .bind(value)
                            .execute(&**db).await.unwrap();
                    }
                )*

                Some(Status::NoContent)
            }

            #[delete($route_id)]
            pub(super) async fn delete(id: i32, db: &State<PgPool>, user: User) -> Option<Status> {
                let (owner,): (i32,) = query_as(&format!("SELECT owner FROM {} WHERE id = $1;", $table))
                    .bind(id)
                    .fetch_optional(&**db).await.unwrap()?;
                if owner != user.id {
                    return Some(Status::Forbidden);
                }

                query(&format!("DELETE FROM {} WHERE id = $1;", $table))
                    .bind(id)
                    .execute(&**db).await.unwrap();

                Some(Status::NoContent)
            }
        }
    };
}

crud!(document, "/document", "/document/<id>", "documents", {
    title: String,
    content: Option<serde_json::Value>,
});

#[derive(Deserialize)]
struct Login {
    login: String,
    password: String,
}

#[post("/login", data = "<login>")]
async fn login(login: Json<Login>, cookie_jar: &CookieJar<'_>, db: &State<PgPool>, identity: Identity<'_>, auth: Option<User>) -> Result<Json<User>, Status> {
    if auth.is_some() {
        return Err(Status::BadRequest);
    }
    let Some(user) = query!(/* language=sql */ "SELECT * FROM users WHERE username = $1 OR email = $1;", login.login)
       .fetch_optional(&**db).await.unwrap() else {
        return Err(Status::Forbidden);
    };
    match ARGON2.verify_password(login.password.as_bytes(), &PasswordHash::new(&*user.password).unwrap()) {
        Err(argon2::password_hash::Error::Password) => return Err(Status::Forbidden),
        Err(err) => Err(err).unwrap(),
        Ok(()) => {},
    }
    let token = query!(/* language=sql */ "
        INSERT INTO sessions (user_id, last_seen, last_ip, last_user_agent)
        VALUES ($1, NOW(), $2, $3)
        RETURNING id;
        ", user.id, identity.ip_string(), identity.user_agent)
        .fetch_one(&**db).await.unwrap();
    cookie_jar.add_private(Cookie::new("session", token.id.to_string()));
    Ok(Json(User {
        id: user.id,
        username: user.username,
        name: user.name,
        email: user.email,
        account_type: user.account_type.try_into().unwrap(),
        created_at: user.created_at,
        require_password_change: user.require_password_change,
    }))
}

#[get("/login")]
async fn current_user(user: User) -> Json<User> {
    Json(user)
}

#[delete("/login")]
async fn logout(_user: User, cookie_jar: &CookieJar<'_>, db: &State<PgPool>) -> Status {
    query!(/* language=postgresql */ "DELETE FROM sessions WHERE id = $1;", cookie_jar.get_private("session").unwrap().value().parse::<i32>().unwrap())
        .execute(&**db).await.unwrap();
    cookie_jar.remove_private(Cookie::named("session"));
    Status::NoContent
}

#[derive(Deserialize)]
struct UpdateUser {
    username: Option<String>,
    name: Option<String>,
    email: Option<EmailAddress>,
    password: Option<String>,
}

#[patch("/login", data = "<update>")]
async fn update_current_user(update: Json<UpdateUser>, user: User, db: &State<PgPool>) -> Status {
    fn is_empty(string: &Option<String>) -> bool {
        string.as_ref().and_then(|s| Some(s.is_empty())).or(Some(false)).unwrap()
    }

    if is_empty(&update.username) || is_empty(&update.name) || is_empty(&update.password) {
        return Status::BadRequest;
    }

    if let Some(username) = &update.username {
        query!(/* language=postgresql */ "UPDATE users SET username = $2 WHERE id = $1;", user.id, username)
            .execute(&**db).await.unwrap();
    }
    if let Some(name) = &update.name {
        query!(/* language=postgresql */ "UPDATE users SET name = $2 WHERE id = $1;", user.id, name)
            .execute(&**db).await.unwrap();
    }
    if let Some(email) = &update.email {
        query!(/* language=postgresql */ "UPDATE users SET email = $2 WHERE id = $1;", user.id, email.to_string())
            .execute(&**db).await.unwrap();
    }
    if let Some(password) = &update.password {
        let salt = SaltString::generate(&mut OsRng);
        let hash = ARGON2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        query!(/* language=postgresql */ "UPDATE users SET password = $2, require_password_change = FALSE WHERE id = $1;", user.id, hash)
            .execute(&**db).await.unwrap();
    }

    Status::NoContent
}
