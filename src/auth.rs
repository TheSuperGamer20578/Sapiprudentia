use std::net::IpAddr;
use anyhow::{anyhow};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use rocket::{async_trait, delete, get, post, Request, Route, routes, State};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{PgPool, query, query_as};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        login,
        current_user,
        logout,
    ];
    pub static ref ARGON2: Argon2<'static> = Argon2::default();
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum AccountType {
    User = 0,
    Admin = 1,
}

impl TryFrom<u8> for AccountType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::User),
            1 => Ok(Self::Admin),
            _ => Err(anyhow!("Invalid account type: {value}")),
        }
    }
}

impl From<i32> for AccountType {
    /// # Panics
    /// If the value is not a valid account type.
    fn from(value: i32) -> Self {
        assert!(!(value < i32::from(u8::MIN) || value > i32::from(u8::MAX)), "Invalid account type: {value}");
        Self::try_from(u8::try_from(value).unwrap()).unwrap()
    }
}

impl Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'d> Deserialize<'d> for AccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'d> {
        Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub account_type: AccountType,
    pub created_at: NaiveDateTime,
    pub require_password_change: bool,
}

impl User {
    async fn from_request_inner(request: &Request<'_>) -> anyhow::Result<Outcome<Self, anyhow::Error>> {
        let Some(token) = request.cookies().get_private("session") else {
            return Ok(Outcome::Error((Status::Unauthorized, anyhow!("No session cookie"))));
        };
        let identity = request.guard::<Identity<'_>>().await.unwrap();
        let Some(user_id) = query!(/* language=postgresql */ "
            UPDATE sessions
            SET last_seen = NOW(), last_ip = $2, last_user_agent = $3
            WHERE id = $1
            RETURNING user_id;
            ", token.value().parse::<i32>()?, identity.ip_string(), identity.user_agent)
            .fetch_optional(request.rocket().state().unwrap()).await? else {
            request.cookies().remove_private(Cookie::from("session"));
            return Ok(Outcome::Error((Status::Unauthorized, anyhow!("Invalid session cookie"))));
        };
        Ok(Outcome::Success(query_as!(Self, /* language=postgresql */ "
            SELECT id, username, name, email, account_type, created_at, require_password_change
            FROM users
            WHERE id = $1
            LIMIT 1;
            ", user_id.user_id)
            .fetch_one(request.rocket().state().unwrap()).await?))
    }
}

impl PartialEq<Self> for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<i32> for User {
    fn eq(&self, other: &i32) -> bool {
        self.id == *other
    }
}

impl Eq for User {}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = anyhow::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match Self::from_request_inner(request).await {
            Ok(outcome) => outcome,
            Err(err) => Outcome::Error((Status::InternalServerError, err)),
        }
    }
}

pub struct Identity<'r> {
    pub ip: Option<IpAddr>,
    pub user_agent: Option<&'r str>,
}

impl<'r> Identity<'r> {
    pub fn ip_string(&self) -> Option<String> {
        self.ip.map(|ip| ip.to_string())
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Identity<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Self {
            ip: request.client_ip(),
            user_agent: request.headers().get_one("User-Agent"),
        })
    }
}

#[derive(Deserialize)]
struct LoginPayload {
    login: String,
    password: String,
}

#[post("/login", data = "<login>")]
async fn login(login: Json<LoginPayload>, cookie_jar: &CookieJar<'_>, db: &State<PgPool>, identity: Identity<'_>, auth: Option<User>) -> Result<Json<User>, Status> {
    if auth.is_some() {
        return Err(Status::BadRequest);
    }
    let user = query!(/* language=postgresql */ "SELECT * FROM users WHERE username = $1 OR email = $1;", login.login)
        .fetch_optional(&**db).await
        .or(Err(Status::InternalServerError))?
        .ok_or(Status::Forbidden)?;
    match ARGON2.verify_password(login.password.as_bytes(), &PasswordHash::new(&*user.password).unwrap()) {
        Err(argon2::password_hash::Error::Password) => return Err(Status::Forbidden),
        Err(_) => return Err(Status::InternalServerError),
        Ok(()) => {},
    }
    let token = query!(/* language=postgresql */ "
        INSERT INTO sessions (user_id, last_seen, last_ip, last_user_agent)
        VALUES ($1, NOW(), $2, $3)
        RETURNING id;
        ", user.id, identity.ip_string(), identity.user_agent)
        .fetch_one(&**db).await.or(Err(Status::InternalServerError))?;
    cookie_jar.add_private(Cookie::new("session", token.id.to_string()));
    Ok(Json(User {
        id: user.id,
        username: user.username,
        name: user.name,
        email: user.email,
        account_type: user.account_type.into(),
        created_at: user.created_at,
        require_password_change: user.require_password_change,
    }))
}

#[get("/login")]
fn current_user(user: User) -> Json<User> {
    Json(user)
}

#[delete("/login")]
async fn logout(auth: Option<User>, cookie_jar: &CookieJar<'_>, db: &State<PgPool>) -> Status {
    if auth.is_none() {
        return Status::BadRequest;
    }
    query!(/* language=postgresql */ "DELETE FROM sessions WHERE id = $1;", cookie_jar.get_private("session").unwrap().value().parse::<i32>().unwrap())
        .execute(&**db).await.unwrap();
    cookie_jar.remove_private(Cookie::from("session"));
    Status::NoContent
}
