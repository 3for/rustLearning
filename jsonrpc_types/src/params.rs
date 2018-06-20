use super::{Error, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde_json;
use serde_json::value::from_value;
use std::fmt;

/// Request parameters
#[derive(Debug, PartialEq, Clone)]
pub enum Params {
    /// Array of values
    Array(Vec<Value>),
    /// Map of values
    Map(serde_json::Map<String, Value>),
    /// No parameters
    None,
}