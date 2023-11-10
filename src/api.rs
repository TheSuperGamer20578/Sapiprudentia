use lazy_static::lazy_static;
use rocket::{get, patch, Route, routes, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::{Deserialize};
use sqlx::{PgPool, query};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        save,
        get_document,
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
async fn get_document(id: i32, db: &State<PgPool>) -> Option<Json<Document>> {
    let document = query!(/* language=sql */ "SELECT title, content FROM documents WHERE id = $1;", id)
        .fetch_optional(&**db).await.unwrap()?;
    Some(Json(Document {
        title: document.title,
        content: document.content,
    }))
}

#[patch("/document/<id>", data = "<document>")]
async fn save(id: i32, db: &State<PgPool>, document: Json<PartialDocument>) -> Status {
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
