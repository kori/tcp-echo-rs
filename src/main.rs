use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];

        stream.read(&mut buffer).unwrap();
        println!("received: {}",  String::from_utf8_lossy(&buffer[..]));

        stream.write(&buffer[..]).unwrap();
    }
}