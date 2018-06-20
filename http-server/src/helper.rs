use futures::sync::oneshot;
use jsonrpc_types::response::Output;

pub enum TransferType {
    /// http output sender
    HTTP((ReqInfo, oneshot::Sender<Output>)),
    /// websocket output sender
    WEBSOCKET((ReqInfo, ws::Sender)),
}

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub jsonrpc: Option<Version>,
    pub id: Id,
}

pub type RpcMap = Arc<Mutex<HashMap<Vec<u8>, TransferType>>>;
pub type ReqSender = Mutex<mpsc::Sender<(String, reqlib::Request)>>;

use std::sync::Arc;
use std::sync::mpsc;