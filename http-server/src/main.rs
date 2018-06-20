
extern crate hyper;

extern crate tokio_io;
extern crate tokio_core;
extern crate net2;

extern crate futures;

extern crate jsonrpc_types;


mod http_server;
mod helper;

use http_server::newServer;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;


use std::io;
use std::net::SocketAddr;



fn main() {

    newServer::start();

    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let listener = http_server::listener(&addr, &handle).unwrap();


}
