use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:3333") {
        Ok(l) => l,
        Err(error) => panic!("Problem creating the file: {:?}", error),
    };
    // This handles all incoming connections.
    for incoming in listener.incoming() {
        thread::spawn(move || {
            let mut stream = match incoming {
                Ok(s) => s,
                Err(error) => panic!("Problem creating a stream: {:?}", error),
            };
            let mut buffer = [0; 512];
            // Having connected, now we need to read the stream multiple times, until the connection is closed.
            loop {
                if let Err(error) = stream.read(&mut buffer) {
                    if let ErrorKind::BrokenPipe = error.kind() {
                        break;
                    }
                }

                println!("received: {}", String::from_utf8_lossy(&buffer[..]));

                if let Err(error) = stream.write(&buffer[..]) {
                    if let ErrorKind::BrokenPipe = error.kind() {
                        break;
                    }
                }
                if let Err(error) = stream.flush() {
                    panic!("Problem flushing: {:?}", error);
                }
            }
        });
    }
}
