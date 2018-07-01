
use std::convert::{From, Into, TryFrom, TryInto};
use util::snappy;
pub use protos::InnerMessage_oneof_content as MsgClass;
use protos::*;
use protobuf::{parse_from_bytes, Message as MessageTrait};

#[derive(Copy, Clone, Debug)]
pub struct TryIntoConvertError(());

#[derive(Copy, Clone, Debug)]
pub struct TryFromConvertError(());

pub type Origin = u32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperateType {
    Broadcast = 0,
    Single = 1,
    Subtract = 2,
}

pub const DEFAULT_OPERATE_TYPE: OperateType = OperateType::Broadcast;
pub use std::u32::MAX as ZERO_ORIGIN;
pub type RawBytes = Vec<u8>;

impl InnerMessage {
    // Param is passed by value, moved
    pub fn set_content(&mut self, v: MsgClass) {
        self.content = Some(v);
    }

    pub fn take_content(&mut self) -> Option<MsgClass> {
        self.content.take()
    }
}

impl From<MsgClass> for InnerMessage {
    fn from(mc: MsgClass) -> Self {
        let mut im = InnerMessage::new();
        im.set_content(mc);
        im
    }
}

/// Message for communication between microservices or nodes.
///
/// # Message in Bytes:
///
/// +---------------------------------------------------------------+
/// | Bytes | Type | Function                                       |
/// |-------+------+------------------------------------------------|
/// |   0   |  u8  | Reserved (For Version?)                        |
/// |   1   |  u8  | Reserved                                       |
/// |   2   |  u8  | Reserved                                       |
/// |-------+------+------------------------------------------------|
/// |   3   |  u4  | Reserved                                       |
/// |       |  u1  | Reserved                                       |
/// |       |  u1  | Compress: true 1, false 0                      |
/// |       |  u2  | OperateType                                    |
/// |-------+------+------------------------------------------------|
/// |  4~7  |  u32 | Origin                                         |
/// |-------+------+------------------------------------------------|
/// |  8~   |      | Payload (Serialized Data with Compress)        |
/// +-------+------+------------------------------------------------+
///
/// We DO NOT have to known the contents of payloads (uncompress and deserialize them) if we just
/// want to distribute them.
/// So we use first 8 bytes to store `OperateType` and `Origin`.
/// And we uncompress and deserialize the payloads only before when we use the contents of them.

#[derive(Clone, Debug)]
pub struct Message {
    raw: Vec<u8>,
}
impl Message {
    pub fn init(operate: OperateType, origin: Origin, mc: MsgClass) -> Self {
        let mut msg = Message { raw: vec![0; 8] };
        msg.set_operate(operate);
        msg.set_origin(origin);
        msg.set_content(mc);
        msg
    }

    #[inline]
    fn let_raw_be_ok(&mut self) {
        if self.raw.len() < 8 {
            let mut vec = vec![0; 8 - self.raw.len()];
            self.raw.append(&mut vec);
        }
    }

    pub fn set_operate(&mut self, ot: OperateType) {
        self.let_raw_be_ok();
        self.raw[3] = (self.raw[3] & 0b11111100) + ((ot as u8) & 0b00000011);
    }

    pub fn set_origin(&mut self, o: Origin) {
        self.let_raw_be_ok();
        self.raw[4] = ((o & 0xFF000000) >> 24) as u8;
        self.raw[5] = ((o & 0x00FF0000) >> 16) as u8;
        self.raw[6] = ((o & 0x0000FF00) >> 8) as u8;
        self.raw[7] = (o & 0x000000FF) as u8;
    }

    pub fn set_content(&mut self, v: MsgClass) {
        let im: InnerMessage = v.into();
        let im_vec: Vec<u8> = im.try_into().unwrap();
        self.raw.drain(8..);
        match snappy::cita_compress_to(&im_vec[..], &mut self.raw) {
            Ok(true) => {
                self.set_compressed(true);
            }
            Ok(false) | Err(_) => {
                self.set_compressed(false);
                self.raw.extend_from_slice(&im_vec[..]);
            }
        }
    }

    fn set_compressed(&mut self, c: bool) {
        self.let_raw_be_ok();
        let c_val: u8 = if c { 0b00000100 } else { 0b00000000 };
        self.raw[3] = (self.raw[3] & 0b11111011) + (c_val & 0b00000100);
    }
}


impl TryFrom<Vec<u8>> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: Vec<u8>) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v })
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: &[u8]) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v.to_vec() })
        }
    }
}

impl<'a> TryFrom<&'a Vec<u8>> for Message {
    type Error = TryFromConvertError;
    fn try_from(v: &Vec<u8>) -> Result<Self, TryFromConvertError> {
        if v.len() < 8 {
            Err(TryFromConvertError(()))
        } else {
            Ok(Message { raw: v.clone() })
        }
    }
}

impl TryInto<Vec<u8>> for Message {
    type Error = TryIntoConvertError;
    fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
        if self.raw.len() < 8 {
            Err(TryIntoConvertError(()))
        } else {
            Ok(self.raw)
        }
    }
}

impl<'a> TryInto<Vec<u8>> for &'a Message {
    type Error = TryIntoConvertError;
    fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
        if self.raw.len() < 8 {
            Err(TryIntoConvertError(()))
        } else {
            Ok(self.raw.clone())
        }
    }
}

macro_rules! impl_convert_for_struct {
    ($( $struct:ident, )+) => {
        $(
            impl_convert_for_struct!($struct);
        )+
    };
    ($struct:ident) => {

        impl<'a> TryFrom<&'a [u8]> for $struct {
            type Error = TryFromConvertError;
            fn try_from(b: &[u8]) -> Result<Self, TryFromConvertError> {
                parse_from_bytes::<$struct>(b).map_err(|_| { TryFromConvertError(()) })
            }
        }

        impl<'a> TryFrom<&'a Vec<u8>> for $struct {
            type Error = TryFromConvertError;
            fn try_from(v: &Vec<u8>) -> Result<Self, TryFromConvertError> {
                parse_from_bytes::<$struct>(v.as_slice()).map_err(|_| { TryFromConvertError(()) })
            }
        }

       impl<'a> TryInto<Vec<u8>> for &'a $struct {
           type Error = TryIntoConvertError;
           fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
               self.write_to_bytes().map_err(|_| { TryIntoConvertError(()) })
           }
       }

       impl TryInto<Vec<u8>> for $struct {
           type Error = TryIntoConvertError;
           fn try_into(self) -> Result<Vec<u8>, TryIntoConvertError> {
               self.write_to_bytes().map_err(|_| { TryIntoConvertError(()) })
           }
       }
    };
}

macro_rules! impl_convert_for_struct_in_msg {
    ($( $struct:ident, )+) => {
        $(
            impl_convert_for_struct_in_msg!($struct);
        )+
    };
    ($struct:ident) => {

       impl Into<MsgClass> for $struct {
           fn into(self) -> MsgClass {
               MsgClass::$struct(self)
           }
       }

        impl Into<Message> for $struct {
            fn into(self) -> Message {
                Message::init(DEFAULT_OPERATE_TYPE, ZERO_ORIGIN, self.into())
            }
        }
    };
}

macro_rules! loop_macro_for_structs {
    ($macro:ident) => {
        $macro!(
            // Generate ALL-PROTOS automatically begin:
            BlockTxHashes,
            BlockTxHashesReq,
            Miscellaneous,
            MiscellaneousReq,
            VerifyBlockReq,
            VerifyBlockResp,
            VerifyTxReq,
            VerifyTxResp,
            AccountGasLimit,
            Block,
            BlockBody,
            BlockHeader,
            BlockTxs,
            BlockWithProof,
            Proof,
            RichStatus,
            SignedTransaction,
            Status,
            Transaction,
            UnverifiedTransaction,
            ActionParams,
            EnvInfo,
            InvokeRequest,
            InvokeResponse,
            KV,
            Log,
            InnerMessage,
            Proposal,
            SignedProposal,
            Vote,
            ConsensusConfig,
            ExecutedHeader,
            ExecutedInfo,
            ExecutedResult,
            LoadRequest,
            LoadResponse,
            LogEntry,
            Receipt,
            ReceiptErrorWithOption,
            ReceiptWithOption,
            RegisterRequest,
            RegisterResponse,
            StateRoot,
            BatchRequest,
            Call,
            Request,
            FullTransaction,
            Response,
            SnapshotReq,
            SnapshotResp,
            SyncRequest,
            SyncResponse,
            // Generate ALL-PROTOS automatically end.
        );
    }
}

macro_rules! loop_macro_for_structs_in_msg {
    ($macro:ident) => {
        $macro!(
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
        );
    }
}

loop_macro_for_structs!(impl_convert_for_struct);
loop_macro_for_structs_in_msg!(impl_convert_for_struct_in_msg);