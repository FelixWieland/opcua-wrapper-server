use std::fs::File;
use std::io::Read;

use crate::crawler::control::CrawlerConfig;

pub fn read_crawler_config(path: &str) -> CrawlerConfig {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: CrawlerConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");

    config
}