use lazy_static::lazy_static;
use rocket::{get, Request, Route, routes};
use rocket::http::{ContentType, Header};
use rocket::response::Responder;
use rocket_dyn_templates::{context, Template};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        react,
    ];
}

#[get("/<_..>", rank = 1000)]
async fn react() -> Template {
    Template::render("react", context! {})
}

// TODO: Refactor this
#[cfg(not(skip_webpack))]
pub struct ServiceWorker(&'static str);

#[cfg(not(skip_webpack))]
impl<'r> Responder<'r, 'r> for ServiceWorker {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'r> {
        self.0.respond_to(request).map(|mut response| {
            response.set_header(Header::new("Service-Worker-Allowed", "/"));
            response.set_header(Header::new("Content-Type", ContentType::JavaScript.to_string()));
            response
        })
    }
}

#[cfg(not(skip_webpack))]
#[get("/static/serviceworker.js")]
pub async fn serviceworker() -> ServiceWorker {
    ServiceWorker(include_str!("../dist/serviceworker.js"))
}
