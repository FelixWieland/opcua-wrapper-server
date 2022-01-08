use std::path::PathBuf;

use opcua_server::{prelude::*};

pub mod crawler;
pub mod schema;

fn main() {
    opcua_console_logging::init();

    let mut server = Server::new(ServerConfig::load(&PathBuf::from("./server.yaml")).unwrap());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write().unwrap();
        address_space
            .register_namespace("urn:opcua-wrapper-server")
            .unwrap()
    };

    println!("Initialize server from schemas");
    let schemas= [
        schema::read::read_schema("./schemas/test_data_provider.json")
    ].to_vec();
    let node_ids = schema::control::initialize_server_from_schemas(&mut server, ns, schemas, &NodeId::objects_folder_id());

    println!("Initialize crawlers");
    let crawler_config = crawler::read::read_crawler_config("./crawler.json");
    crawler_config.create_and_run_all_crawlers(&mut server, &node_ids);

    println!("Starting server");
    server.run();
}