use std::io::{Read, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("My addr is {}", stream.local_addr()?);
    println!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer).expect("failed to read"));
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

// listener.incoming()は新規のコネクションが構築されるまでメインスレッドをブロックして、
// イテレータを返すのでそれでループを回している
// Iterating over it is equivalent to calling accept in a loop.
// tcpstreamはサーバーソケットのaccept後やクライアントからの接続に使う。

