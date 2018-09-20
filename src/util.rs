use comrak::{ ComrakOptions, markdown_to_html };
use std::fs;

pub fn get_post_list() -> Vec<String> {
    let mut posts: Vec<String> = Vec::new();
    let paths = fs::read_dir("posts/").unwrap();
    for path in paths {
        let filename = path.unwrap().file_name();
        if let Some(name) = filename.to_str().unwrap().split(".").next() {
            posts.push(name.to_string());
        }
    }
    posts
}

pub fn render_markdown(content: &mut String) -> String {
    let mut markdown_opt = ComrakOptions::default();
    markdown_opt.safe = false;
    let markdown = markdown_to_html(&content, &markdown_opt);
    markdown
}