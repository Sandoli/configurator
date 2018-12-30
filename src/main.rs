extern crate zmq;
extern crate prost;
#[macro_use]
extern crate prost_derive;

use zmq::{Context, Message, SocketType};
use prost::Message;

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

fn client(ctx : &mut Context, addr : &str)
{
	let sock = ctx.socket(SocketType::REQ).unwrap();
	let _ = sock.connect(addr).unwrap();
	let mut message = messages::Message::default();
	let mut key_value = messages::KeyValue::default();
	key_value.key = "Hello".to_string();
	key_value.value = "Dominique".to_string();
	message.key_values = Some(key_value);
	//let payload = "Hello dominique!".to_string();
	println!("-> {} {}!", key_value.key, key_value.value);
	let mut buf = Vec::new();
	message.encode(buf);
	let msg = Message::from_slice(&buf);
	//msg.data = payload.into_bytes();
	let _ = sock.send(msg, 0);
	if let Ok(msg) = sock.recv_msg(0) {
	    let contents = msg.as_str().expect("Not a UTF-8 string");
	    println!("<- {}", contents);
	}
}

fn server(ctx : &mut Context, addr : &str)
{
	let sock = ctx.socket(SocketType::REP).unwrap();
	let _ = sock.bind(addr).unwrap();
	loop {
	    if let Ok(msg) = sock.recv_msg(0) {
		let contents = msg.as_str().expect("Not a UTF-8 string");
		println!("<- {}", contents);
		let response = Message::from(msg);
		let _ = sock.send(response, 0);
	    }
	}
}

