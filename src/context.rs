use page::Page;
use siteconfig::SiteConfig;
use post::Post;
use util;

#[derive(Serialize)]
pub struct Context {
    pub site: SiteConfig,
    pub post: Option<Post>,
    pub current_page: Option<String>,
}


impl Context {
    pub fn new() -> Context {
        Context {
            site: SiteConfig::new(),
            post: None,
            current_page: None,
        }
    }
}

#[derive(Serialize)]
pub struct PageContext {
    pub site: SiteConfig,
    pub page: Option<Page>,
    pub current_page: Option<String>,
}


impl PageContext {
    pub fn new() -> PageContext {
        PageContext {
            site: SiteConfig::new(),
            page: None,
            current_page: None,
        }
    }
}

#[derive(Serialize)]
pub struct PostListContext {
    pub site: SiteConfig,
    pub posts: Vec<String>,
    pub current_page: Option<String>,
}

impl PostListContext {
    pub fn new() -> PostListContext {
        PostListContext {
            site: SiteConfig::new(),
            posts: util::get_post_list(),
            current_page: None,
        }
    }
}