#![feature(try_from)]


extern crate protobuf;
extern crate grpc;
extern crate tls_api;
extern crate util;

pub mod protos;
pub use protos::*;

pub mod router;

mod autoimpl;
pub use autoimpl::{Message, MsgClass, OperateType, Origin, RawBytes, TryFromConvertError, TryIntoConvertError,
                   ZERO_ORIGIN};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
