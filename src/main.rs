#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rocket;
extern crate comrak;
#[macro_use]
extern crate rocket_contrib;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io::Read;
use rocket::response::Failure;
use rocket::http::Status;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::fs::File;
use std::path::Path;
use comrak::{markdown_to_html, ComrakOptions};
use rocket_contrib::Template;

mod context;
mod siteconfig;
mod post;

use post::Post;
use context::Context;

#[get("/")]
fn index() -> Template {
    let arg = Context::new();
    Template::render("index", &arg) 
}

#[get("/posts/<post_name>")]
fn posts(post_name: String) -> Result<Template, Failure> {
    let mut content = String::new();
    let filename = format!("posts/{}.md", post_name);
    if let Ok(mut result) = File::open(filename) {
        if let Ok(_) = result.read_to_string(&mut content) {
            let mut markdown_opt = ComrakOptions::default();
            markdown_opt.safe = false;
            let markdown = markdown_to_html(&content, &markdown_opt);
            let mut post = Post {
                title: post_name,
                content: markdown,
            };
            let mut context = Context::new();
            context.post = Some(post);
            Ok(Template::render("posts", &context))
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
