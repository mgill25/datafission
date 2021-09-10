// minimal arrow impl

use std::collections::{BTreeMap, HashMap};
use crate::datatype::DataType;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

// Schema -> Field -> DataType
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Schema {
    fields: Vec<Field>,
    metadata: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Field {
    name: String,
    data_type: DataType,
    nullable: bool,
    metadata: Option<BTreeMap<String, String>>
}

impl Field {
    pub fn new(name: &str, data_type: DataType, nullable: bool) -> Self {
        Field {
            name: name.to_string(),
            data_type,
            nullable,
            metadata: None
        }
    }
}