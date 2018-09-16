use siteconfig::SiteConfig;

#[derive(Serialize)]
pub struct Context {
    site: SiteConfig
}


impl Context {
    pub fn new() -> Context {
        Context {
            site: SiteConfig::new()
        }
    }
}