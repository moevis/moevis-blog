#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate comrak;
extern crate rocket_contrib;

use rocket::response::content::Html;
use std::io::Read;
use rocket::response::Failure;
use rocket::http::Status;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::fs::File;
use std::path::Path;
use comrak::{markdown_to_html, ComrakOptions};
use rocket::response::content;
use rocket_contrib::Template;


#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/posts/<post_name>")]
fn posts(post_name: String) -> Result<content::Html<String>, Failure> {
    let mut content = String::new();
    let filename = format!("posts/{}.md", post_name);
    if let Ok(mut result) = File::open(filename) {
        if let Ok(_) = result.read_to_string(&mut content) {
            let mut markdown_opt = ComrakOptions::default();
            markdown_opt.safe = false;
            let markdown = markdown_to_html(&content, &markdown_opt);
            Ok(Html(markdown))
        } else {
            Err(Failure(Status::InternalServerError))
        }
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            posts,
            static_file,
        ])
        .attach(Template::fairing())
        .launch();
}
