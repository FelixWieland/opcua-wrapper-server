use serde::Deserialize;
use opcua_server::{prelude::*};
use crate::{crawler::test_data_provider, schema::control::ExtendedNodeId};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlerConfig {
    pub targets: Vec<CrawlTarget>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrawlTarget {
    pub variant: String,
    pub interval: i32,
    pub endpoint: String,
    pub schema_path: String
}

pub trait Crawler {
    fn start(&self, server: &mut Server, node_ids: &Vec<ExtendedNodeId>);
}

impl CrawlTarget {
    pub fn create_crawler(self) -> Box<dyn Crawler> {
        match self.variant.as_str() {
        "test_data_provider" => Box::new(test_data_provider::TestDataProviderCrawler::new(self)),
        _ => panic!("Cant create {} crawler - NotFound", self.variant)
        }
    }
}
