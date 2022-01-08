use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub browse_name: Option<String>,
    pub display_name: Option<String>,
    pub node_id: Option<String>,
    pub children: Option<Vec<Schema>>
}

pub fn read_schema(path: &str) -> Schema {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let schema: Schema = serde_json::from_str(&data).expect("JSON was not well-formatted");
    schema
}

impl Schema {
    pub fn get_node_id(&self) -> String {
        match &self.node_id {
            Some(node_id) => node_id.to_owned(),
            None => "-".to_owned(),
        }
    }

    pub fn get_browse_name(&self) -> String {
        match &self.browse_name {
            Some(browse_name) => browse_name.to_owned(),
            None => self.get_node_id(),
        }
    }

    pub fn get_display_name(&self) -> String {
        match &self.display_name {
            Some(display_name) => display_name.to_owned(),
            None => self.get_browse_name(),
        }
    }
}