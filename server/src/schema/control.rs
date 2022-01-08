use opcua_server::{prelude::*};
use crate::schema::read::Schema;

#[derive(Clone)]
pub struct ExtendedNodeId {
    pub opcua_node_id: NodeId,
    pub schema: Schema
}

pub fn initialize_server_from_schema(server: &mut Server, ns: u16, schema: Schema, parent_id: &NodeId) -> Vec<ExtendedNodeId> {
    match schema.children.clone() {
        Some(children) => {
            let address_space = server.address_space();
            let mut address_space = address_space.write().unwrap();
            let parent = address_space
                .add_folder(schema.get_browse_name(), schema.get_display_name(), parent_id)
                .unwrap();
            drop(address_space);
            initialize_server_from_schemas(server, ns, children.to_vec(), &parent)
        },
        None => {
            let address_space = server.address_space();
            let mut address_space = address_space.write().unwrap();
            let opcua_node_id = NodeId::new(ns, schema.get_node_id());
            let _ = address_space.add_variables(
                vec![
                    Variable::new(&opcua_node_id, schema.get_browse_name(), schema.get_display_name(), 0 as i32),
                ],
                parent_id,
            );
            drop(address_space);
            [ExtendedNodeId{
                opcua_node_id,
                schema,
            }].to_vec()
        },
    }
}

pub fn initialize_server_from_schemas(server: &mut Server, ns: u16, schemas: Vec<Schema>, parent_id: &NodeId) -> Vec<ExtendedNodeId> {
    let mut node_ids = vec![]; 
    for schema in schemas {
        node_ids.append(&mut initialize_server_from_schema(server, ns, schema, parent_id))
    }
    node_ids
}
