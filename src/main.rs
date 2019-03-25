use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};
use std::{thread, str};

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("My addr is {}", stream.local_addr()?);
    println!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..nbytes]).expect("failed to read"));
        stream.write(&buffer[..nbytes])?;
        stream.flush()?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:33333").expect("Error. failed to bind to the address");
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
