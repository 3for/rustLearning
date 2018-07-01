#![feature(extern_prelude)]
#![feature(try_from)]


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
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate error;
extern crate unicase;

extern crate uuid;
#[macro_use]
extern crate log;

mod http_server;
mod helper;
mod response;
mod config;


use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpListener;


use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::{channel};
use std::collections::HashMap;

use http_server::Server;
use util::Mutex;

use config::NewTxFlowConfig;
use std::thread;
use std::time::SystemTime;
use libproto::request as reqlib;
use uuid::Uuid;
use std::sync::mpsc::{ Sender};
use libproto::Message;
use libproto::router::{MsgType, RoutingKey, SubModules};
use std::convert::TryInto;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let allow_origin : Option<String> = Some("*".to_string() );
    let timeout = 3;
    let backlog_capacity = 1000;
    let tx_flow_config : NewTxFlowConfig = NewTxFlowConfig{
        count_per_batch: 30,
        buffer_duration: 30000000,
    };


    //used for buffer message
    let (tx_relay, rx_relay) = channel();
    //Init pubsub
    let (tx_pub, rx_pub) = channel();

    let responses = Arc::new(Mutex::new(HashMap::with_capacity(backlog_capacity)));
    let http_responses = Arc::clone(&responses);


    if(true) {
        let mut reqtest = create_request();
        reqtest.set_block_number(true);

        println!("reqtest: {:?}", reqtest);

        //dispatch
        thread::spawn(move || {
            let mut new_tx_request_buffer : Vec<reqlib::Request> = Vec::new();
            let mut time_stamp = SystemTime::now();
            loop {
                if let Ok(res) = rx_relay.try_recv() {
                    let (topic, req): (String, reqlib::Request) = res;
                    println!("zyd topic: {}", topic);
                    println!("zyd req: {:?}", req);
                    forward_service(
                        topic,
                        req,
                        &mut new_tx_request_buffer,
                        &mut time_stamp,
                        &tx_pub,
                        &tx_flow_config,
                    );
                } else {
                    /*if !new_tx_request_buffer.is_empty() {
                        batch_forward_new_tx(&mut new_tx_request_buffer, &mut time_stamp, &tx_pub);
                    }*/
                    thread::sleep(Duration::new(0, tx_flow_config.buffer_duration));
                }
            }
        });



        // Create the event loop that will drive this server
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let timeout = Duration::from_secs(timeout);

        let http_responses = Arc::clone(&http_responses);
        let tx = tx_relay.clone();

        let listener = http_server::listener(&addr, &handle).unwrap();
        //Server::start(core, listener, tx, http_responses, timeout, &allow_origin);

        //以线程方式运行，不堵塞主线程
        let _ = thread::Builder::new()
            .name(format!("worker{}", 0))
            .spawn(move || {
                Server::start(core, listener, tx, http_responses, timeout, &allow_origin);
            })
            .unwrap();

    }

    loop {
        //let (key, msg) = rx_sub.recv().unwrap();
        let (key, msg) = rx_pub.recv().unwrap();
        println!("key: {}", key);
        println!("msg: {:?}", msg);

    }
}

fn forward_service(
    topic: String,
    req: reqlib::Request,
    new_tx_request_buffer: &mut Vec<reqlib::Request>,
    time_stamp: &mut SystemTime,
    tx_pub: &Sender<(String, Vec<u8>)>,
    config: &NewTxFlowConfig,
) {
    if RoutingKey::from(&topic) != routing_key!(Jsonrpc >> RequestNewTx) {
        let data: Message = req.into();
        tx_pub.send((topic, data.try_into().unwrap())).unwrap();
    } else {
        new_tx_request_buffer.push(req);
        trace!(
            "New tx is pushed and has {} new tx and buffer time cost is {:?}",
            new_tx_request_buffer.len(),
            time_stamp.elapsed().unwrap()
        );
        if new_tx_request_buffer.len() > config.count_per_batch
            || time_stamp.elapsed().unwrap().subsec_nanos() > config.buffer_duration
            {
               // batch_forward_new_tx(new_tx_request_buffer, time_stamp, tx_pub); //zouyudi.tiaoshi.20180629.
            }
    }
}

pub fn create_request() -> reqlib::Request {
    let request_id = Uuid::new_v4().as_bytes().to_vec();
    let mut request = reqlib::Request::new();
    request.set_request_id(request_id);
    request
}
