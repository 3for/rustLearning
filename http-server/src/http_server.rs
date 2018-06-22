//extern crate hyper;

use hyper::{StatusCode, Body};
use hyper::header::{ Headers, ContentType, AccessControlAllowOrigin};
use hyper::server::{Http, NewService, Request, Response, Service};

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;

use futures::future::{ Future};

use net2;

use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use std::sync::{ Arc, mpsc };

use helper::{ ReqInfo, ReqSender, RpcMap, TransferType};
use jsonrpc_types::method::{self, MethodHandler};

use libproto::request as reqlib;
use util::Mutex;

const TCP_BACKLOG: i32 = 1024;

struct Inner {
    pub tx: ReqSender,
    pub responses: RpcMap,
    pub timeout: Duration,
    pub reactor_handle: Handle,
    pub method_handler: method::MethodHandler,
    pub http_headers: Headers,
}

pub struct Server {
    inner: Arc<Inner>,
}

pub struct NewServer {
    inner: Arc<Inner>,
}

impl NewService for NewServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Server;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Server {
            inner: Arc::clone(&self.inner),
        })
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let http_headers = self.inner.http_headers.clone();

        Box::new(futures::future::ok(
            Response::new()
//                .with_headers(http_headers)
//                .with_status(StatusCode::Ok)
                .with_body("hello world zouyudi...\n")
        ))
    }
}

impl Server {
    pub fn start(
        core: Core,
        listener: TcpListener,
        tx: mpsc::Sender<(String, reqlib::Request)>,
        responses: RpcMap,
        timeout: Duration,
        allow_origin: &Option<String>,
    ) {
        let mut headers = Headers::new();
        let origin = parse_origin(allow_origin);
        headers.set(ContentType::json());
        headers.set(origin);

        let new_service = NewServer {
            inner: Arc::new(Inner {
                tx: Mutex::new(tx),
                responses: responses,
                timeout: timeout,
                reactor_handle: core.handle(),
                method_handler: method::MethodHandler,
                http_headers: headers,
            }),
        };
        let server = Http::new()
            .sleep_on_errors(Some(Duration::from_millis(50)))
            .keep_alive(true)
            .bind_listener(core, listener, new_service)
            .unwrap();
        server.run().unwrap();
    }
}

fn parse_origin(origin: &Option<String>) -> AccessControlAllowOrigin {
    match origin.as_ref().map(|s| s.trim().as_ref()) {
        Some("*") => AccessControlAllowOrigin::Any,
        None | Some("") | Some("null") => AccessControlAllowOrigin::Null,
        Some(origin) => AccessControlAllowOrigin::Value(origin.to_string()),
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