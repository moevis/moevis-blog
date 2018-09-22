use rocket::response::Redirect;
use rocket::response::Flash;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::outcome::IntoOutcome;
use rocket_contrib::Template;
use std::collections::HashMap;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String
}

#[derive(Debug)]
struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, !> {
        request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

#[get("/login")]
pub fn login_page() -> Template {
    let arg: HashMap<String, String> = HashMap::new();
    Template::render("login", &arg)
}

#[post("/login", data = "<login>")]
fn login(mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    if login.get().username == "aaa" && login.get().password == "bbb" {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Redirect::to("/"))
    } else {
        Err(Flash::error(Redirect::to("/login"), "Invalid username/password."))
    }
}