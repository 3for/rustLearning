use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as SError;
use request::Version;
use Id;
use bytes::Bytes;
use error::Error;

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