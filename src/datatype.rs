// Minimal DataTypes supported by Arrow
// [the physical memory layout of Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout).
use std::fmt;
use std::fmt::Formatter;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::schema::Field;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataType {
    Null,
    Boolean,
    Int8,
    UInt8,
    Binary,
    Utf8,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}