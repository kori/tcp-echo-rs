use std::io::prelude::*;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    // This handles all incoming connections.
    for stream in listener.incoming() {
        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 512];
            // Having connected, now we need to read multiple times, until the connection is closed.
            loop {
                stream.read(&mut buffer).unwrap();
                println!("received: {}", String::from_utf8_lossy(&buffer[..]));

                stream.write(&buffer[..]).unwrap();
                stream.flush().unwrap();
            }
        });
    }
}
