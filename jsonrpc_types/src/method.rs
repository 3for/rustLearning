use Call;
use Error;
use Params;

use libproto::request as reqlib;
use uuid::Uuid;

pub mod method {
    pub const CITA_BLOCK_BUMBER: &str = "cita_blockNumber";
    pub const CITA_GET_BLOCK_BY_HASH: &str = "cita_getBlockByHash";
    pub const CITA_GET_BLOCK_BY_NUMBER: &str = "cita_getBlockByNumber";
    pub const CITA_GET_TRANSACTION: &str = "cita_getTransaction";
    pub const CITA_SEND_TRANSACTION: &str = "cita_sendTransaction";
    pub const CITA_GET_TRANSACTION_PROOF: &str = "cita_getTransactionProof";
    pub const CITA_GET_META_DATA: &str = "cita_getMetaData";
    pub const NET_PEER_COUNT: &str = "net_peerCount";
    /// Executes a new message call immediately without creating a transaction on the block chain.
    /// Parameters
    /// 1. Object - The transaction call object
    /// from: DATA, 20 Bytes - (optional) The address the transaction is sent from.
    /// to: DATA, 20 Bytes - The address the transaction is directed to.
    /// data: DATA - (optional) Hash of the method signature and encoded parameters.
    /// 2. QUANTITY|TAG - integer block height, or the string "latest" or "earliest".
    pub const ETH_GET_TRANSACTION_COUNT: &str = "eth_getTransactionCount";
    pub const ETH_GET_CODE: &str = "eth_getCode";
    pub const ETH_GET_ABI: &str = "eth_getAbi";
    pub const ETH_CALL: &str = "eth_call";
    pub const ETH_GET_LOGS: &str = "eth_getLogs";
    pub const ETH_GET_TRANSACTION_RECEIPT: &str = "eth_getTransactionReceipt";
    pub const ETH_GET_BALANCE: &str = "eth_getBalance";

    /// filter
    pub const ETH_NEW_FILTER: &str = "eth_newFilter";
    pub const ETH_NEW_BLOCK_FILTER: &str = "eth_newBlockFilter";
    pub const ETH_UNINSTALL_FILTER: &str = "eth_uninstallFilter";
    pub const ETH_GET_FILTER_CHANGES: &str = "eth_getFilterChanges";
    pub const ETH_GET_FILTER_LOGS: &str = "eth_getFilterLogs";
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MethodHandler;

impl MethodHandler {
    pub fn request(&self, rpc: &Call) -> Result<reqlib::Request, Error> {
        match rpc.method.as_str() {
            method::CITA_BLOCK_BUMBER => self.block_number(rpc),
            /*method::NET_PEER_COUNT => self.peer_count(rpc),
            method::CITA_GET_BLOCK_BY_HASH => self.get_block_by_hash(rpc),
            method::CITA_GET_BLOCK_BY_NUMBER => self.get_block_by_number(rpc),
            method::CITA_GET_TRANSACTION => self.get_transaction(rpc),
            method::CITA_GET_TRANSACTION_PROOF => self.get_transaction_proof(rpc),
            method::CITA_GET_META_DATA => self.get_meta_data(rpc),
            method::ETH_CALL => self.call(rpc),
            method::ETH_GET_LOGS => self.get_logs(rpc),
            method::ETH_GET_TRANSACTION_RECEIPT => self.get_transaction_receipt(rpc),
            method::ETH_GET_TRANSACTION_COUNT => self.get_transaction_count(rpc),
            method::ETH_GET_CODE => self.get_code(rpc),
            method::ETH_GET_ABI => self.get_abi(rpc),
            method::ETH_GET_BALANCE => self.get_balance(rpc),
            method::CITA_SEND_TRANSACTION => self.send_transaction(rpc),

            method::ETH_NEW_FILTER => self.new_filter(rpc),

            method::ETH_NEW_BLOCK_FILTER => self.new_block_filter(rpc),

            method::ETH_UNINSTALL_FILTER => self.uninstall_filter(rpc),
            method::ETH_GET_FILTER_CHANGES => self.get_filter_changes(rpc),
            method::ETH_GET_FILTER_LOGS => self.get_filter_logs(rpc),*/

            _ => Err(Error::method_not_found()),
        }
    }


    pub fn block_number(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 0 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let mut request = self.create_request();
        request.set_block_number(true);
        Ok(request)
    }

    pub fn params_len(&self, params: &Option<Params>) -> usize {
        if let &Some(ref params) = params {
            params.len()
        } else {
            0
        }
    }

    pub fn create_request(&self) -> reqlib::Request {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = reqlib::Request::new();
        request.set_request_id(request_id);
        request
    }

}