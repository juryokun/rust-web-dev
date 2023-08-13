#[macro_use]
extern crate rocket;

use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, handlebars, Template};
use serde_json::json;
// use serde::{Deserialize, Serialize};
// use rocket::http::RawStr;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use mockall::*;
use std::env;

pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub username: String,
    pub password: String,
}

// ユーザーのデータモデル
#[serde(crate = "rocket::serde")]
#[derive(Serialize)]
struct User {
    username: String,
    password: String,
}

// ログインフォームのデータモデル
#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_form = form.into_inner();

    let mut db = PgDatabase::new();
    let user = db.find_by_username(&login_form.username);
    // let hashed_password = hash(dummy_user.password, DEFAULT_COST).unwrap();
    // let password_matched = verify(&login_form.password, &hashed_password).map_err(|_| {
    //     Flash::error(
    //         Redirect::to("/login_result"),
    //         "Invalid username or password.",
    //     )
    // })?;
    let password_matched = true;

    // if login_form.username == user.username && password_matched {
    if password_matched {
        // ログイン成功時の処理
        // セッションにログイン状態を保存するなど

        Ok(Flash::success(
            Redirect::to("/login_result"),
            "Login successful.",
        ))
    } else {
        // ログイン失敗時の処理
        Err(Flash::error(
            Redirect::to("/login_result"),
            "Invalid username or password.",
        ))
    }
}

struct UserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    fn new(user_repository: R) -> Self {
        Self {
            user_repository: user_repository,
        }
    }
    fn login(&mut self, login_form: &LoginForm) -> Result<bool, ()> {
        let user = self.user_repository.find_by_username(&login_form.username);
        let password_matched = true;
        Ok(password_matched)
    }
}

struct PgDatabase {
    connection: PgConnection,
}
impl PgDatabase {
    fn new() -> Self {
        Self {
            connection: establish_connection(),
        }
    }
}

#[automock]
trait UserRepository {
    fn find_by_username(&mut self, username: &str) -> Result<Users, diesel::result::Error>;
}

impl UserRepository for PgDatabase {
    fn find_by_username(&mut self, username: &str) -> Result<Users, diesel::result::Error> {
        use self::schema::users::dsl::*;
        let results = users.find(username).first::<Users>(&mut self.connection);
        results
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/reception")]
pub fn reception() -> Template {
    Template::render("reception", context! {foo: ""})
}

#[get("/login_result")]
pub fn login_result(flash: Option<FlashMessage>) -> Template {
    let context = flash.map(|msg| {
        let kind = msg.kind();
        let message = msg.message();

        json!({
            "kind": kind,
            "message": message
        })
    });

    Template::render("login_result", &context)
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
        foo: String,
    }

    let config: Config = figment.extract().expect("config");
    println!("{:?}", config);

    rocket
}

#[cfg(test)]
mod test {
    // use super::rocket;
    // use rocket::http::{ContentType, Status};
    // use rocket::local::blocking::Client;
    // use rocket::uri;
    //
    // #[test]
    // fn index() {
    //     let client = Client::tracked(rocket()).expect("valid rocket instance");
    //     let mut response = client.get(uri!(super::index)).dispatch();
    //
    //     assert_eq!(response.status(), Status::Ok);
    //     assert_eq!(response.content_type(), Some(ContentType::Plain));
    //     assert!(response
    //         .headers()
    //         .get_one("X-Content-Type-Options")
    //         .is_some());
    //     assert_eq!(response.into_string().unwrap(), "Hello, world!");
    // }
    //
    // #[test]
    // fn login() {
    //     let client = Client::tracked(rocket()).expect("valid rocket instance");
    //     let mut response = client
    //         .post(uri!(super::login))
    //         .header(ContentType::Form)
    //         .body("user_cd=take&password=yama")
    //         .dispatch();
    //
    //     assert_eq!(response.status(), Status::Ok);
    //     assert!(response
    //         .headers()
    //         .get_one("X-Content-Type-Options")
    //         .is_some());
    //     assert!(response.into_string().unwrap().contains("Success!"))
    // }
    //
    use super::*;

    #[test]
    fn test_login() {
        let test_user = LoginForm {
            username: "yama".to_string(),
            password: "take".to_string(),
        };

        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_find_by_username()
            .with(predicate::eq("yama"))
            .times(1)
            .returning(|_| {
                Ok(Users {
                    username: "yama".to_string(),
                    password: "take".to_string(),
                })
            });

        let mut service = UserService::new(mock_user_repository);
        let result = service.login(&test_user);
        assert_eq!(result, Ok(true));
    }
}
