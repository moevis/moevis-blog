use siteconfig::SiteConfig;
use post::Post;
use util;

#[derive(Serialize)]
pub struct Context {
    pub site: SiteConfig,
    pub post: Option<Post>,
}


impl Context {
    pub fn new() -> Context {
        Context {
            site: SiteConfig::new(),
            post: None,
        }
    }
}

#[derive(Serialize)]
pub struct PostListContext {
    pub site: SiteConfig,
    pub posts: Vec<String>,
}

impl PostListContext {
    pub fn new() -> PostListContext {
        PostListContext {
            site: SiteConfig::new(),
            posts: util::get_post_list()
        }
    }
}