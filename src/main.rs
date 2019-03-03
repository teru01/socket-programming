extern crate rand;

use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::str;

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        // let sleep = Duration::from_secs(*thread_rng().choose(&[1,2,3,4,5]).unwrap());
        let sleep = Duration::from_secs(0);
        println!("sleeping for {:?} before replying", sleep);
        std::thread::sleep(sleep);
        print!("{}", str::from_utf8(&buffer).expect("failed to read"));
        stream.write(&buffer[..nbytes])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => { eprintln!("error: {}", e)},
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
