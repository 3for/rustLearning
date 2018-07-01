use std::convert::{From, Into};
use std::fmt;

#[macro_export]
macro_rules! routing_key {
    ([$( $sm:tt >> $mt:tt ),+ ,]) => {{
        routing_key![[ $( $sm >> $mt ),+ ]]
    }};
    ([$( $sm:tt >> $mt:tt ),+]) => {{
        vec![ $( routing_key!($sm >> $mt) ),+ ]
    }};
    ($sm:ident >> $mt:ident) => {
        RoutingKey (
            SubModules::$sm,
            MsgType::$mt,
        )
    };
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SubModules {
    Jsonrpc,
    Net,
    Chain,
    Consensus,
    Auth,
    Executor,
    Synchronizer,
    Snapshot,
    All,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MsgType {
    // Generate MSG-PROTOS struct automatically begin:
    RawBytes,
    Request,
    Response,
    SyncRequest,
    SyncResponse,
    Status,
    RichStatus,
    SignedProposal,
    Block,
    BlockWithProof,
    BlockHeader,
    BlockTxs,
    BlockTxHashes,
    BlockTxHashesReq,
    VerifyTxReq,
    VerifyTxResp,
    VerifyBlockReq,
    VerifyBlockResp,
    ExecutedResult,
    SnapshotReq,
    SnapshotResp,
    Miscellaneous,
    MiscellaneousReq,
    // Generate MSG-PROTOS struct automatically end.
    All,
    Unknown,
    // TODO This is a issue left over by history.
    //      The Request is too big (send from Jsonrpc).
    //      To remove follow items should be better.
    LocalSync,
    RequestNewTx,
    RequestNewTxBatch,
    RequestNet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoutingKey(pub SubModules, pub MsgType);

const UNKNOWN: &str = "__unknown__";
pub const SUBMODULES_UNKNOWN: SubModules = SubModules::Unknown;
pub const MSGTYPE_UNKNOWN: MsgType = MsgType::Unknown;
pub const ROUTINGKEY_UNKNOWN: RoutingKey = RoutingKey(SUBMODULES_UNKNOWN, MSGTYPE_UNKNOWN);

impl fmt::Display for SubModules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Please use the same rules to name the string
                &SubModules::Jsonrpc => "jsonrpc",
                &SubModules::Net => "net",
                &SubModules::Chain => "chain",
                &SubModules::Consensus => "consensus",
                &SubModules::Auth => "auth",
                &SubModules::Executor => "executor",
                &SubModules::Synchronizer => "synchronizer",
                &SubModules::Snapshot => "snapshot",
                &SubModules::All => "*",
                &SubModules::Unknown => UNKNOWN,
            }
        )
    }
}

impl fmt::Display for MsgType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Generate MSG-PROTOS display automatically begin:
                &MsgType::RawBytes => "raw_bytes",
                &MsgType::Request => "request",
                &MsgType::Response => "response",
                &MsgType::SyncRequest => "sync_request",
                &MsgType::SyncResponse => "sync_response",
                &MsgType::Status => "status",
                &MsgType::RichStatus => "rich_status",
                &MsgType::SignedProposal => "signed_proposal",
                &MsgType::Block => "block",
                &MsgType::BlockWithProof => "block_with_proof",
                &MsgType::BlockHeader => "block_header",
                &MsgType::BlockTxs => "block_txs",
                &MsgType::BlockTxHashes => "block_tx_hashes",
                &MsgType::BlockTxHashesReq => "block_tx_hashes_req",
                &MsgType::VerifyTxReq => "verify_tx_req",
                &MsgType::VerifyTxResp => "verify_tx_resp",
                &MsgType::VerifyBlockReq => "verify_block_req",
                &MsgType::VerifyBlockResp => "verify_block_resp",
                &MsgType::ExecutedResult => "executed_result",
                &MsgType::SnapshotReq => "snapshot_req",
                &MsgType::SnapshotResp => "snapshot_resp",
                &MsgType::Miscellaneous => "miscellaneous",
                &MsgType::MiscellaneousReq => "miscellaneous_req",
                // Generate MSG-PROTOS display automatically end.
                &MsgType::LocalSync => "sync",
                &MsgType::All => "*",
                &MsgType::Unknown => UNKNOWN,
                &MsgType::RequestNewTx => "request_new_tx",
                &MsgType::RequestNewTxBatch => "request_new_tx_batch",
                &MsgType::RequestNet => "request_net",
            }
        )
    }
}

impl fmt::Display for RoutingKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl<'a> From<&'a str> for SubModules {
    fn from(s: &'a str) -> SubModules {
        match s {
            "jsonrpc" => SubModules::Jsonrpc,
            "net" => SubModules::Net,
            "chain" => SubModules::Chain,
            "consensus" => SubModules::Consensus,
            "auth" => SubModules::Auth,
            "executor" => SubModules::Executor,
            "synchronizer" => SubModules::Synchronizer,
            "snapshot" => SubModules::Snapshot,
            "*" => SubModules::All,
            _ => SubModules::Unknown,
        }
    }
}

impl<'a> From<&'a str> for MsgType {
    fn from(s: &'a str) -> MsgType {
        match s {
            // Generate MSG-PROTOS from_str automatically begin:
            "raw_bytes" => MsgType::RawBytes,
            "request" => MsgType::Request,
            "response" => MsgType::Response,
            "sync_request" => MsgType::SyncRequest,
            "sync_response" => MsgType::SyncResponse,
            "status" => MsgType::Status,
            "rich_status" => MsgType::RichStatus,
            "signed_proposal" => MsgType::SignedProposal,
            "block" => MsgType::Block,
            "block_with_proof" => MsgType::BlockWithProof,
            "block_header" => MsgType::BlockHeader,
            "block_txs" => MsgType::BlockTxs,
            "block_tx_hashes" => MsgType::BlockTxHashes,
            "block_tx_hashes_req" => MsgType::BlockTxHashesReq,
            "verify_tx_req" => MsgType::VerifyTxReq,
            "verify_tx_resp" => MsgType::VerifyTxResp,
            "verify_block_req" => MsgType::VerifyBlockReq,
            "verify_block_resp" => MsgType::VerifyBlockResp,
            "executed_result" => MsgType::ExecutedResult,
            "snapshot_req" => MsgType::SnapshotReq,
            "snapshot_resp" => MsgType::SnapshotResp,
            "miscellaneous" => MsgType::Miscellaneous,
            "miscellaneous_req" => MsgType::MiscellaneousReq,
            // Generate MSG-PROTOS from_str automatically end.
            "sync" => MsgType::LocalSync,
            "*" => MsgType::All,
            "request_new_tx" => MsgType::RequestNewTx,
            "request_new_tx_batch" => MsgType::RequestNewTxBatch,
            "request_net" => MsgType::RequestNet,
            _ => MsgType::Unknown,
        }
    }
}

impl<'a> From<&'a str> for RoutingKey {
    fn from(s: &'a str) -> RoutingKey {
        let mut items = s.split('.').take(3);
        match (items.next(), items.next(), items.next()) {
            (Some(sm), Some(mt), None) => RoutingKey(SubModules::from(sm), MsgType::from(mt)),
            _ => ROUTINGKEY_UNKNOWN,
        }
    }
}

macro_rules! impl_some_traits {
    ($struct:ident) => (
        impl From<String> for $struct {
            fn from(s: String) -> $struct {
                $struct::from(s.as_str())
            }
        }

        impl<'a> From<&'a String> for $struct {
            fn from(s: &'a String) -> $struct {
                $struct::from(s.as_str())
            }
        }

        impl Into<String> for $struct {
            fn into(self) -> String {
                self.to_string()
            }
        }
    );
}

impl_some_traits!(SubModules);
impl_some_traits!(MsgType);
impl_some_traits!(RoutingKey);