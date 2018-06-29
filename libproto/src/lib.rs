extern crate protobuf;
extern crate grpc;
extern crate tls_api;

pub mod protos;
pub use protos::*;

pub mod router;

mod autoimpl;
pub use autoimpl::{Message, TryFromConvertError, TryIntoConvertError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
