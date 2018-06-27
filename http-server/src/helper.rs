use futures::sync::oneshot;
use jsonrpc_types::response::Output;
use util::Mutex;
use jsonrpc_types::request::Version;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use jsonrpc_types::Id;
use libproto::request as reqlib;
use libproto::router::{MsgType, RoutingKey, SubModules};

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

impl ReqInfo {
    pub fn new(jsonrpc: Option<Version>, id: Id) -> ReqInfo {
        ReqInfo {
            jsonrpc: jsonrpc,
            id: id,
        }
    }
}

pub type RpcMap = Arc<Mutex<HashMap<Vec<u8>, TransferType>>>;
pub type ReqSender = Mutex<mpsc::Sender<(String, reqlib::Request)>>;

pub fn select_topic(method: &str) -> String {
    if method.starts_with("cita_send") {
        routing_key!(Jsonrpc >> RequestNewTx).into()
    } else if method.starts_with("cita") || method.starts_with("eth") {
        routing_key!(Jsonrpc >> Request).into()
    } else if method.starts_with("net_") {
        routing_key!(Jsonrpc >> RequestNet).into()
    } else {
        "jsonrpc".to_string()
    }
}