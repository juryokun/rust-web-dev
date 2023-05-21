#[macro_use]
extern crate rocket;

use rocket::{get, launch, routes};
use rocket_dyn_templates::{Template, handlebars, context};
use rocket::serde::Deserialize;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/templating/<arg_foo>")]
pub fn login(arg_foo: &str) -> Template {
    Template::render("index", context! {
        foo: arg_foo,
    })
}

#[get("/reception")]
pub fn reception() -> Template {
    Template::render("reception", context! {foo: ""})
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .mount("/", routes![index, reception])
        .attach(Template::fairing());

    let figment = rocket.figment();
    #[derive(Deserialize, Debug)]
    #[serde(crate = "rocket::serde")]
    struct Config {
        port: u16,
        foo: String
    }

    let config: Config = figment.extract().expect("config");
    println!("{:?}", config);

    rocket
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::uri;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get(uri!(super::index)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert!(response.headers().get_one("X-Content-Type-Options").is_some());
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
