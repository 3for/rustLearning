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

impl Params {

    pub fn len(&self) -> usize {
        match *self {
            Params::Array(ref vec) => vec.len(),
            Params::Map(ref map) => map.len(),
            Params::None => 0,
        }
    }
}

impl Serialize for Params {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            Params::Array(ref vec) => vec.serialize(serializer),
            Params::Map(ref map) => map.serialize(serializer),
            Params::None => ([0u8; 0]).serialize(serializer),
        }
    }
}

struct ParamsVisitor;

impl<'a> Deserialize<'a> for Params {
    fn deserialize<D>(deserializer: D) -> Result<Params, D::Error>
        where
            D: Deserializer<'a>,
    {
        deserializer.deserialize_any(ParamsVisitor)
    }
}

impl<'a> Visitor<'a> for ParamsVisitor {
    type Value = Params;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map or sequence")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'a>,
    {
        let mut values = Vec::new();

        while let Some(value) = visitor.next_element()? {
            values.push(value);
        }

        if values.is_empty() {
            Ok(Params::None)
        } else {
            Ok(Params::Array(values))
        }
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'a>,
    {
        let mut values = serde_json::Map::new();

        while let Some((key, value)) = visitor.next_entry()? {
            values.insert(key, value);
        }

        Ok(if values.is_empty() {
            Params::None
        } else {
            Params::Map(values)
        })
    }
}