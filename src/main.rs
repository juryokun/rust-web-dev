#[macro_use]
extern crate rocket;

use rocket::{get, launch, routes};
use rocket_dyn_templates::{Template, handlebars, context};
use rocket::serde::Deserialize;
use rocket::form::Form;
// use rocket::http::RawStr;
use mockall::*;

#[derive(FromForm, Debug)]
pub struct LoginInfo {
    user_cd: String,
    password: String,
}

#[derive(Debug)]
struct Authenticator {
    login_info: LoginInfo,
    repository: AuthRepository,
}

impl Authenticator {
    fn new(login_info: LoginInfo, repository: AuthRepository) -> Self {
        Self {
            login_info: login_info,
            repository: repository,
        }
    }

    fn authenticate(&self) -> Result<(), String> {
        self.repository.check(&self)
    }
}

#[derive(Debug)]
enum DBConnection {
    Database,
    Vector,
}

#[automock]
trait AuthRepositoryTrait {
    fn check(&self, authenticator: &Authenticator) -> Result<(), String>;
}

#[derive(Debug)]
struct AuthRepository {
    db: DBConnection
}

impl AuthRepository {
    fn new() -> Self {
        Self {
            db: DBConnection::Vector
        }
    }
}

impl AuthRepositoryTrait for AuthRepository {
    fn check(&self, authenticator: &Authenticator) -> Result<(), String> {
        if authenticator.login_info.user_cd == "take" && authenticator.login_info.password == "yama" {
            Ok(())
        } else {
            Err("Not Found".to_string())
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login", data = "<login_info>")]
pub fn login(login_info: Form<LoginInfo>) -> Template {
    let auth_repository = AuthRepository::new();
    let _auth = Authenticator::new(login_info.into_inner(), auth_repository);
    match _auth.authenticate() {
        Ok(_) => return Template::render("login_result", context! { result: "Success!", message: "" }),
        Err(v) => return Template::render("login_result", context! { result: "Faild!", message: v }),
    }
}

#[get("/reception")]
pub fn reception() -> Template {
    Template::render("reception", context! {foo: ""})
}

#[get("/login_result")]
pub fn login_result() -> Template {
    Template::render("login_result", context! {})
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .mount("/", routes![index, reception, login, login_result])
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

    #[test]
    fn login() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.post(uri!(super::login))
            .header(ContentType::Form)
            .body("user_cd=take&password=yama")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.headers().get_one("X-Content-Type-Options").is_some());
        assert!(response.into_string().unwrap().contains("Success!"))
    }

    #[test]
    fn test_authenticate() {
    }
}
