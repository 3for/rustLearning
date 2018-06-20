


/// Error object as defined in Spec
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Error {
    /// Code
    pub code: ErrorCode,
    /// Message
    pub message: String,
    /// Optional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}