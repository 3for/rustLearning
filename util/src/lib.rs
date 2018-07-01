#![feature(try_from)]
extern crate parking_lot;
extern crate libc;

pub use parking_lot::{Mutex};

pub mod snappy;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
