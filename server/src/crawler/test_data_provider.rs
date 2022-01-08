use crate::{crawler::control, schema::control::ExtendedNodeId};
use async_std::task;
use opcua_server::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct TestDataProviderCrawler {
    pub interval: i32,
    pub endpoint: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestDataProviderCrawlerResponse {
    pub node_id: String,
    pub value: f64,
}

impl TestDataProviderCrawler {
    pub fn new(target: control::CrawlTarget) -> TestDataProviderCrawler {
        TestDataProviderCrawler {
            interval: target.interval,
            endpoint: target.endpoint,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<TestDataProviderCrawlerResponse>, surf::Error> {
        let res = surf::post(self.endpoint.to_owned()).send().await;
        match res {
            Ok(mut response) => {
                let res: Result<Vec<TestDataProviderCrawlerResponse>, surf::Error> =
                    response.body_json().await;
                res
            }
            Err(err) => Err(err),
        }
    }
}

impl control::Crawler for TestDataProviderCrawler {
    fn start(&self, server: &mut Server, node_ids: &Vec<ExtendedNodeId>) {
        let address_space = server.address_space();
        let data = Arc::new(Mutex::new((0, true)));
        let node_ids = node_ids.to_owned();
        let crawler = self.to_owned();

        let mut node_map: HashMap<String, ExtendedNodeId> = HashMap::new();
        for node in node_ids {
            node_map.insert(node.schema.get_node_id(), node);
        }

        server.add_polling_action(self.interval.try_into().unwrap(), move || {
            let mut data = data.lock().unwrap();
            data.0 += 1;
            data.1 = !data.1;
            let mut address_space = address_space.write().unwrap();
            let now = DateTime::now();
            let res = task::block_on(crawler.fetch());

            match res {
                Ok(response) => {
                    for item in response {
                        let node_id = node_map.get(&item.node_id);
                        match node_id {
                            Some(node_id) => {
                                let opcua_node_id = &node_id.opcua_node_id;
                                let _ = 
                                    address_space.set_variable_value(opcua_node_id.clone(), data.0 as i32, &now, &now);
                            },
                            None => (),
                        };
                    }
                },
                Err(err) => println!("{:?}", err),
            }
        });
    }
}
