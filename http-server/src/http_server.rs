//extern crate hyper;

use hyper::{Body, Request, Response, Server};
use hyper::header::{ Headers};

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;

use net2;

use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use helper::{ ReqInfo, ReqSender, RpcMap, TransferType};
use jsonrpc_types::method::{self, MethodHandler};

const TCP_BACKLOG: i32 = 1024;

struct Inner {
    pub tx: ReqSender,
    pub responses: RpcMap,
    pub timeout: Duration,
    pub reactor_handle: Handle,
    pub method_handler: method::MethodHandler,
    pub http_headers: Headers,
}


pub struct newServer {

}

impl newServer {
    pub fn start(
    ) {

    }
}

pub fn listener(addr: &SocketAddr, handle: &Handle) -> io::Result<TcpListener> {
    let listener = match *addr {
        SocketAddr::V4(_) => net2::TcpBuilder::new_v4()?,
        SocketAddr::V6(_) => net2::TcpBuilder::new_v6()?,
    };
    configure_tcp(&listener)?;
    listener.reuse_address(true)?;
    listener.bind(addr)?;
    listener
        .listen(TCP_BACKLOG)
        .and_then(|l| TcpListener::from_listener(l, addr, handle))
}

fn configure_tcp(tcp: &net2::TcpBuilder) -> io::Result<()> {
    use net2::unix::*;
    tcp.reuse_port(true)?;
    Ok(())
}