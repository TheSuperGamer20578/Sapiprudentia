use async_graphql::{Object, SimpleObject};
use chrono::NaiveDateTime;

#[derive(SimpleObject)]
pub struct Session {
    /// The session ID.
    pub id: i32,
    
    #[graphql(skip)]
    pub user_id: i32,
    
    /// The IP address of the last request made with this session.
    #[graphql(name = "ip")]
    pub last_ip: Option<String>,
    
    /// The user agent of the last request made with this session.
    #[graphql(name = "userAgent")]
    pub last_user_agent: Option<String>,
    
    /// The date and time the session was created.
    pub created_at: NaiveDateTime,
    
    /// The date and time of the last request made with this session.
    pub last_seen: NaiveDateTime,
}
