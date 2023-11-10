use lazy_static::lazy_static;
use rocket::{get, Route, routes};
use rocket_dyn_templates::{context, Template};

lazy_static! {
    pub static ref ROUTES: Vec<Route> = routes![
        react,
    ];
}

#[get("/<_..>")]
async fn react() -> Template {
    Template::render("react", context! {})
}
