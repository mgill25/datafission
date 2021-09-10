// Minimal DataTypes supported by Arrow
// [the physical memory layout of Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout).
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum _DataType {
    Null,
    Boolean,
    Int8,
    UInt8,
    Binary,
    Utf8,
}

impl fmt::Display for _DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
