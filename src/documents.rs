use lazy_static::lazy_static;
use rocket::{get, Route, routes, State};
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use sqlx::{PgPool, query};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        new,
        show,
    ];
}

#[get("/new")]
async fn new(db: &State<PgPool>) -> Redirect {
    let document = query!(/* language=sql */ "INSERT INTO documents DEFAULT VALUES RETURNING id;")
        .fetch_one(&**db).await.unwrap();
    Redirect::to(format!("/document/{}", document.id))
}

#[get("/<id>")]
async fn show(id: i32, db: &State<PgPool>) -> Template {
    let document = query!(/* language=sql */ "SELECT id, title FROM documents WHERE id = $1;", id)
        .fetch_one(&**db).await.unwrap();
    let documents: Vec<_> = query!(/* language=sql */ "SELECT id, title FROM documents;")
        .fetch_all(&**db).await.unwrap()
        .into_iter()
        .map(|document| context! {
            id: document.id,
            title: if document.title.is_empty() {"Untitled".into()} else {document.title},
        })
        .collect();
    Template::render("template", context! {
        new_document: document.title.is_empty(),
        document_id: document.id,
        document_title: document.title,
        documents: documents,
    })
}
