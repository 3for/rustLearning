#![feature(extern_prelude)]
extern crate hyper;

extern crate tokio_io;
extern crate tokio_core;
extern crate net2;

extern crate futures;

extern crate jsonrpc_types;
extern crate util;
#[macro_use]
extern crate libproto;

extern crate serde_json;

extern crate error;
extern crate unicase;

mod http_server;
mod helper;
mod response;


use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;


use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::{channel};
use std::collections::HashMap;

use http_server::Server;
use util::Mutex;


fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let allow_origin : Option<String> = Some("*".to_string() );
    let timeout = 3;
    let backlog_capacity = 1000;

    //used for buffer message
    let (tx_relay, rx_relay) = channel();

    let responses = Arc::new(Mutex::new(HashMap::with_capacity(backlog_capacity)));
    let http_responses = Arc::clone(&responses);


    if(true) {
        // Create the event loop that will drive this server
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let timeout = Duration::from_secs(timeout);

        let http_responses = Arc::clone(&http_responses);
        let tx = tx_relay.clone();

        let listener = http_server::listener(&addr, &handle).unwrap();
        Server::start(core, listener, tx, http_responses, timeout, &allow_origin);
    }

    loop {

    }
}
