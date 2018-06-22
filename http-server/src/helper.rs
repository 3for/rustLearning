use futures::sync::oneshot;
use jsonrpc_types::response::Output;
use util::Mutex;
use jsonrpc_types::request::Version;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use jsonrpc_types::Id;
use libproto::request as reqlib;

pub enum TransferType {
    /// http output sender
    HTTP((ReqInfo, oneshot::Sender<Output>)),
    /*/// websocket output sender
    WEBSOCKET((ReqInfo, ws::Sender)),*/
}

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub jsonrpc: Option<Version>,
    pub id: Id,
}

pub type RpcMap = Arc<Mutex<HashMap<Vec<u8>, TransferType>>>;
pub type ReqSender = Mutex<mpsc::Sender<(String, reqlib::Request)>>;

