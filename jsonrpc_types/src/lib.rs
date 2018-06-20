extern crate serde;
extern crate serde_derive;
extern crate serde_json;

pub mod method;
pub mod response;
pub mod request;

mod id;
mod params;
pub mod error;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
