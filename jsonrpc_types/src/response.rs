use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as SError;
use request::Version;
use Id;
use bytes::Bytes;
use error::Error;

use serde_json::{from_value, Value};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultBody {
    /*BlockNumber(U256),
    FullBlock(Block),
    #[serde(rename = "null")] Null,
    Receipt(Receipt),
    Transaction(RpcTransaction),
    TxResponse(TxResponse),
    PeerCount(U256),
    CallResult(Bytes),
    Logs(Vec<Log>),
    TranactionCount(U256),
    ContractCode(Bytes),
    ContractAbi(Bytes),
    FilterId(U256),
    UninstallFliter(bool),
    FilterChanges(FilterChanges),
    FilterLog(Vec<Log>),
    TxProof(Bytes),
    MetaData(MetaData),
    Balance(U256),*/
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcFailure {
    #[serde(skip_serializing_if = "Option::is_none")] pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: Error,
}

impl From<Error> for RpcFailure {
    fn from(err: Error) -> Self {
        RpcFailure {
            id: Id::Null,
            jsonrpc: None,
            error: err,
        }
    }
}

impl RpcFailure {
    pub fn from_options(id: Id, jsonrpc: Option<Version>, err: Error) -> RpcFailure {
        RpcFailure {
            id: id,
            jsonrpc: jsonrpc,
            error: err,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcSuccess {
    #[serde(skip_serializing_if = "Option::is_none")] pub jsonrpc: Option<Version>,
    pub id: Id,
    pub result: ResultBody,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Output {
    /// Success
    Success(RpcSuccess),
    /// Failure
    Failure(RpcFailure),
}

impl<'a> Deserialize<'a> for Output {
    fn deserialize<D>(deserializer: D) -> Result<Output, D::Error>
        where
            D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(Output::Failure)
            .or_else(|_| from_value(v).map(Output::Success))
            .map_err(|_| D::Error::custom("")) // types must match
    }
}

impl Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            Output::Success(ref s) => s.serialize(serializer),
            Output::Failure(ref f) => f.serialize(serializer),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RpcResponse {
    /// Single response
    Single(Output),
    /// Response to batch request (batch of responses)
    Batch(Vec<Output>),
}

impl<'a> Deserialize<'a> for RpcResponse {
    fn deserialize<D>(deserializer: D) -> Result<RpcResponse, D::Error>
        where
            D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(RpcResponse::Batch)
            .or_else(|_| from_value(v).map(RpcResponse::Single))
            .map_err(|_| D::Error::custom("")) // types must match
    }
}

impl Serialize for RpcResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            RpcResponse::Single(ref o) => o.serialize(serializer),
            RpcResponse::Batch(ref b) => b.serialize(serializer),
        }
    }
}