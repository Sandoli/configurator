extern crate zmq;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use zmq::{Context, Message, SocketType};
use prost::Message as ProtoMsg;
use std::ops::Deref;

// Include the `items` module, which is generated from items.proto.
pub mod messages {
    include!(concat!(env!("OUT_DIR"), "/configurator.messages.rs"));
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} (client|server)", args[0]);
        return;
    }

    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    if args[1] == "client" {
        println!("ZeroMQ client connecting to {}", addr);
	client(&mut ctx, &addr);
    }
    else {
        println!("ZeroMQ server listening on {}", addr);
	server(&mut ctx, &addr);
    }
}

impl messages::ConfigMessage {
	fn add_key_value<T: Into<String>>(&mut self, key : T, value : T) {
		let mut key_value = messages::KeyValue::default();
		key_value.key = key.into();
		key_value.value = value.into();

		println!("-> {} {}!", key_value.key, key_value.value);	

		self.key_values.push(key_value);
	}
}


fn client(ctx : &mut Context, addr : &str) {
	let sock = ctx.socket(SocketType::REQ).unwrap();
	let _ = sock.connect(addr).unwrap();
	let mut message = messages::ConfigMessage::default();
	message.add_key_value("Hello", "Dominique");
	message.add_key_value("Ca", "va ?");
	
	let mut buf = Vec::with_capacity(message.encoded_len());
	message.encode(&mut buf).unwrap();
	let msg = Message::from_slice(&buf);
	let _ = sock.send(msg, 0);
	if let Ok(msg) = sock.recv_msg(0) {
		let message: messages::ConfigMessage = ProtoMsg::decode(msg.deref()).unwrap();
		for kv in message.key_values.iter() {
			println!("<- {} {}!", kv.key, kv.value);
		}

	    //let contents = msg.as_str().expect("Not a UTF-8 string");
	    //println!("<- {}", contents);
	}
}

fn server(ctx : &mut Context, addr : &str)
{
	let sock = ctx.socket(SocketType::REP).unwrap();
	let _ = sock.bind(addr).unwrap();
	loop {
	    if let Ok(msg) = sock.recv_msg(0) {
		let message: messages::ConfigMessage = ProtoMsg::decode(msg.deref()).unwrap();
		for kv in message.key_values.iter() {
			println!("<- {} {}!", kv.key, kv.value);
		}

		let response = Message::from(msg);
		let _ = sock.send(response, 0);
	    }
	}
}

