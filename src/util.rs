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