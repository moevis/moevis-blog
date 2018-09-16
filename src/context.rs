use siteconfig::SiteConfig;
use post::Post;

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