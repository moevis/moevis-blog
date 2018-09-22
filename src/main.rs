#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![feature(never_type)]

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

use admin_router::static_rocket_route_info_for_login;
use admin_router::static_rocket_route_info_for_login_page;
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
mod util;
mod page;
mod admin_router;

use post::Post;
use context::{ Context, PostListContext, PageContext };
use std::collections::HashMap;
use util::render_markdown;
use page::Page;
use admin_router::{ login_page };

#[get("/")]
fn index() -> Template {
    let arg = PostListContext::new();
    Template::render("index", &arg) 
}

#[get("/pages/posts", rank=1)]
fn post_pages() -> Template {
    let arg = PostListContext::new();
    Template::render("index", &arg) 
}

#[get("/pages/<page_name>", rank=2)]
fn site_pages(page_name: String) -> Result<Template, Failure> {
    let filename = format!("pages/{}.md", page_name);
    let mut content = String::new();
    if let Ok(mut result) = File::open(filename) {
        if let Ok(_) = result.read_to_string(&mut content) {
            let markdown = render_markdown(&mut content);
            let mut page = Page {
                name: page_name.clone(),
                content: markdown,
            };
            let mut context = PageContext::new();
            context.page = Some(page);
            context.current_page = Some(page_name);
            Ok(Template::render("pages", &context))
        } else {
            Err(Failure(Status::InternalServerError))
        }
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[get("/posts/<post_name>")]
fn posts(post_name: String) -> Result<Template, Failure> {
    let mut content = String::new();
    let filename = format!("posts/{}.md", post_name);
    if let Ok(mut result) = File::open(filename) {
        if let Ok(_) = result.read_to_string(&mut content) {
            let markdown = render_markdown(&mut content);
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

#[catch(404)]
fn not_found() -> Template {
    let map: HashMap<String, String> = HashMap::new();
    Template::render("error/not_found", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            posts,
            static_file,
            site_pages,
            post_pages,
            login_page,
            login,
        ])
        .attach(Template::fairing())
        .catch(catchers![not_found])
        .launch();
}
