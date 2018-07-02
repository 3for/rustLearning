extern crate amqp;

use amqp::{protocol, Basic, Channel, Consumer, Session, Table};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

pub struct Handler {
    tx: Sender<(String, Vec<u8>)>,
}

impl Handler {
    pub fn new(tx: Sender<(String, Vec<u8>)>) -> Self {
        Handler { tx: tx }
    }
}

impl Consumer for Handler {
    fn handle_delivery(
        &mut self,
        channel: &mut Channel,
        deliver: protocol::basic::Deliver,
        _: protocol::basic::BasicProperties,
        body: Vec<u8>,
    ) {
        let _ = self.tx.send((deliver.routing_key, body));
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}

pub const AMQP_URL: &'static str = "AMQP_URL";

pub fn start_rabbitmq(name: &str, keys: Vec<String>, tx: Sender<(String, Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    let amqp_url = std::env::var(AMQP_URL).expect(format!("{} must be set", AMQP_URL).as_str());
    let mut session = match Session::open_url(&amqp_url) {
        Ok(session) => session,
        Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
    };

    let mut channel = session.open_channel(1).ok().expect("Can't open channel");
    let _ = channel.basic_prefetch(10);
    channel
        .exchange_declare(
            "cita",
            "topic",
            false,
            true,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    channel
        .queue_declare(name.clone(), false, true, false, false, false, Table::new())
        .unwrap();

    for key in keys {
        channel
            .queue_bind(name.clone(), "cita", &key, false, Table::new())
            .unwrap();
    }
    let callback = Handler::new(tx);
    //queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    channel
        .basic_consume(
            callback,
            name.clone(),
            "",
            false,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    // thread recv msg from mq
    let _ = thread::Builder::new()
        .name("subscriber".to_string())
        .spawn(move || {
            channel.start_consuming();
            let _ = channel.close(200, "Bye");
        });

    let mut session = match Session::open_url(&amqp_url) {
        Ok(session) => session,
        Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
    };
    let mut channel = session.open_channel(1).ok().expect("Can't open channel");
    let _ = channel.basic_prefetch(10);
    channel
        .exchange_declare(
            "cita",
            "topic",
            false,
            true,
            false,
            false,
            false,
            Table::new(),
        )
        .unwrap();

    // thread send msg to mq
    let _ = thread::Builder::new()
        .name("publisher".to_string())
        .spawn(move || {
            loop {
                let ret = rx.recv();
                if ret.is_err() {
                    break;
                }
                let (routing_key, msg) = ret.unwrap();
                let _ = channel.basic_publish(
                    "cita",
                    &routing_key,
                    false,
                    false,
                    protocol::basic::BasicProperties {
                        content_type: Some("text".to_string()),
                        ..Default::default()
                    },
                    msg,
                );
            }
            let _ = channel.close(200, "Bye");
        });
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
