use serde::{Deserialize, Deserializer, Serialize, Serializer};
use Id;
use Params;
use serde_json::{from_value, Value};
use serde::de::Error;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Version {
    #[serde(rename = "1.0")] V1,
    #[serde(rename = "2.0")] V2,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Call {
    pub jsonrpc: Option<Version>,
    pub method: String,
    pub id: Id,
    pub params: Option<Params>,
}

/// Represents jsonrpc request.
#[derive(Debug, Clone, PartialEq)]
pub enum RpcRequest {
    /// Single request (call)
    Single(Call),
    /// Batch of requests (calls)
    Batch(Vec<Call>),
}

impl Serialize for RpcRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            RpcRequest::Single(ref call) => call.serialize(serializer),
            RpcRequest::Batch(ref calls) => calls.serialize(serializer),
        }
    }
}

impl<'a> Deserialize<'a> for RpcRequest {
    fn deserialize<D>(deserializer: D) -> Result<RpcRequest, D::Error>
        where
            D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(RpcRequest::Batch)
            .or_else(|_| from_value(v).map(RpcRequest::Single))
            .map_err(|_| D::Error::custom("")) // unreachable, but types must match
    }
}