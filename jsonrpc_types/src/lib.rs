
#![feature(custom_attribute)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod method;
pub mod response;
pub mod request;

mod id;
mod params;
pub mod error;
pub mod bytes;

pub use self::id::*;
pub use self::error::*;
pub use serde_json::Value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
