use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
pub struct Subject {
    /// The ID of the subject.
    pub id: i32,
    
    #[graphql(skip)]
    pub owner: i32,
    
    /// The name of the subject.
    pub name: String,
    
    /// The class of the subject.
    pub class: String,
    
    /// Whether the subject should appear in navigation.
    pub active: bool,
    
    /// The Google Classroom ID of the subject.
    pub google_classroom_id: Option<String>,
}
