use std::net::IpAddr;
use anyhow::{anyhow};
use argon2::Argon2;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use rocket::{async_trait, Request};
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{query, query_as};

lazy_static! {
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
        if value < u8::MIN as i32 || value > u8::MAX as i32 {
            panic!("Invalid account type: {value}");
        }
        Self::try_from(value as u8).unwrap()
    }
}

impl Serialize for AccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'d> Deserialize<'d> for AccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'d> {
        Ok(Self::try_from(u8::deserialize(deserializer)?).map_err(serde::de::Error::custom)?)
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
            return Ok(Outcome::Failure((Status::Unauthorized, anyhow!("No session cookie"))));
        };
        let identity = request.guard::<Identity<'_>>().await.unwrap();
        let Some(user_id) = query!(/* language=postgresql */ "
            UPDATE sessions
            SET last_seen = NOW(), last_ip = $2, last_user_agent = $3 WHERE id = $1
            RETURNING user_id;
            ", token.value().parse::<i32>()?, identity.ip_string(), identity.user_agent)
            .fetch_optional(request.rocket().state().unwrap()).await? else {
            request.cookies().remove_private(Cookie::named("session"));
            return Ok(Outcome::Failure((Status::Unauthorized, anyhow!("Invalid session cookie"))));
        };
        Ok(Outcome::Success(query_as!(Self, /* language=postgresql */ "
            SELECT id, username, name, email, account_type, created_at, require_password_change
            FROM users
            WHERE id = $1;
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
            Err(err) => Outcome::Failure((Status::InternalServerError, err)),
        }
    }
}

pub struct Identity<'r> {
    pub ip: Option<IpAddr>,
    pub user_agent: Option<&'r str>,
}

impl<'r> Identity<'r> {
    pub fn ip_string(&self) -> Option<String> {
        self.ip.and_then(|ip| Some(ip.to_string()))
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
