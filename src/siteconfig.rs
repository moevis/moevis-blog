use config::Config;
use std::collections::HashMap;
use config::File;

lazy_static! {
    pub static ref SITE_CONFIG: HashMap<String, String> = {
        let mut config = Config::default();
        config.merge(File::with_name("site_config.yml")).unwrap();
        let mut m: HashMap<String, String> = HashMap::new();

        let site_config_keys = ["site_url", "site_title", "site_subtitle", "site_author"];
        for key in site_config_keys.iter() {
            m.insert(
                key.to_string(),
                config.get(key).expect(
                    &format!("{} is required in site_config.yml", key))
            );
        }
        m
    };
}

#[derive(Serialize, Debug)]
pub struct SiteConfig {
    pub site_url: String,
    pub site_title: String,
    pub site_subtitle: String,
    pub site_author: String,
}

impl SiteConfig {
    pub fn new() -> SiteConfig {
        SiteConfig {
            site_url: SITE_CONFIG.get("site_url").unwrap().to_string(),
            site_title: SITE_CONFIG.get("site_title").unwrap().to_string(),
            site_subtitle: SITE_CONFIG.get("site_subtitle").unwrap().to_string(),
            site_author: SITE_CONFIG.get("site_author").unwrap().to_string(),
        }
    }
}