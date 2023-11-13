use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use email_address::EmailAddress;
use lazy_static::lazy_static;
use rocket::{delete, get, patch, post, Route, routes, State};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::{Deserialize};
use sqlx::{PgPool, query};
use crate::auth::{ARGON2, Identity, User};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        get_document,
        patch_document,
        login,
        current_user,
        logout,
        update_current_user,
    ];
}

#[derive(Serialize, Deserialize)]
struct Document {
    title: String,
    content: Option<serde_json::Value>,
}

impl TryFrom<PartialDocument> for Document {
    type Error = ();

    fn try_from(value: PartialDocument) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title.ok_or(())?,
            content: value.content,
        })
    }
}

#[derive(Deserialize)]
struct PartialDocument {
    title: Option<String>,
    content: Option<serde_json::Value>,
}

impl From<Document> for PartialDocument {
    fn from(value: Document) -> Self {
        Self {
            title: Some(value.title),
            content: value.content,
        }
    }
}

#[get("/document/<id>")]
async fn get_document(id: i32, db: &State<PgPool>, user: User) -> Option<Result<Json<Document>, Status>> {
    let document = query!(/* language=sql */ "SELECT title, content, owner FROM documents WHERE id = $1;", id)
        .fetch_optional(&**db).await.unwrap()?;
    if document.owner != user.id {
        return Some(Err(Status::Forbidden));
    }
    Some(Ok(Json(Document {
        title: document.title,
        content: document.content,
    })))
}

#[patch("/document/<id>", data = "<document>")]
async fn patch_document(id: i32, db: &State<PgPool>, document: Json<PartialDocument>, user: User) -> Status {
    let owner = match query!(/* language=sql */ "SELECT owner FROM documents WHERE id = $1;", id)
        .fetch_optional(&**db).await.unwrap() {
        Some(document) => document.owner,
        None => return Status::NotFound,
    };
    if owner != user.id {
        return Status::Forbidden;
    }
    match &*document {
        PartialDocument {title: Some(title), content: Some(content)} => {
            query!(/* language=sql */ "UPDATE documents SET title = $1, content = $2 WHERE id = $3;", title, content, id)
                .execute(&**db).await.unwrap();
        }
        PartialDocument {title: Some(title), content: None} => {
            query!(/* language=sql */ "UPDATE documents SET title = $1 WHERE id = $2;", title, id)
                .execute(&**db).await.unwrap();

        }
        PartialDocument {title: None, content: Some(content)} => {
            query!(/* language=sql */ "UPDATE documents SET content = $1 WHERE id = $2;", content, id)
                .execute(&**db).await.unwrap();

        }
        PartialDocument {title: None, content: None} => {}
    };
    Status::NoContent
}

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
