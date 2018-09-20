use config::Config;
use std::collections::HashMap;
use config::File;

lazy_static! {
    pub static ref SITE_CONFIG: SiteConfig = {
        let mut config = Config::default();
        config.merge(File::with_name("site_config.yml")).unwrap();

        let site_config = SiteConfig {
            site_author:  config.get("site_author").expect("site_author is required in site_config.yml"),
            site_title: config.get("site_title").expect("site_title is required in site_config.yml"),
            site_subtitle: config.get("site_subtitle").expect("site_subtitle is required in site_config.yml"),
            site_url: config.get("site_url").expect("site_url is required in site_config.yml"),
            pages: config.get("pages").expect("pages is required in site_config.yml"),
        };
        site_config
    };
}

#[derive(Serialize, Debug, Clone)]
pub struct SiteConfig {
    pub site_url: String,
    pub site_title: String,
    pub site_subtitle: String,
    pub site_author: String,
    pub pages: Vec<String>,
}

impl SiteConfig {
    pub fn new() -> SiteConfig {
        SITE_CONFIG.clone()
    }
}