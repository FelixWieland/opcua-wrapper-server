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

    println!("Initialize server");
    let crawler_config = crawler::read::read_crawler_config("./config.json");
    
    for target in crawler_config.targets {
        let schema = schema::read::read_schema(&target.schema_path);
        let node_ids = schema::control::initialize_server_from_schema(&mut server, ns, schema, &NodeId::objects_folder_id());
        let crawler = target.create_crawler();
        crawler.start(&mut server, &node_ids)
    }
    
    println!("Starting server");
    server.run();
}