// minimal arrow impl

use crate::datatype::_DataType;
use serde_derive::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// Schema -> Field -> DataType
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct _Schema {
    fields: Vec<_Field>,
    metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct _Field {
    name: String,
    data_type: _DataType,
    nullable: bool,
    metadata: Option<BTreeMap<String, String>>,
}

impl _Field {
    #[allow(dead_code)]
    pub fn new(name: &str, data_type: _DataType, nullable: bool) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            nullable,
            metadata: None,
        }
    }
}