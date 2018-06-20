use serde::{ Serialize, Serializer};


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