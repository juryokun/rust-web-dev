#[macro_use]
extern crate rocket;

use rocket::{get, launch, routes};
use rocket_dyn_templates::{Template, handlebars, context};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/templating/<arg_foo>")]
pub fn templating(arg_foo: &str) -> Template {
    Template::render("index", context! {
        foo: arg_foo,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/login", routes![templating])
        .attach(Template::fairing())
}

