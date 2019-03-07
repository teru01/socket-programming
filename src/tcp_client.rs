use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let remote = "127.0.0.1:33333".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect.");
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    loop {
        let file = File::open("rust.html").unwrap();
        let mut buf_reader = BufReader::new(file);

        let mut input = String::new();
        buf_reader.read_to_string(&mut input).unwrap();

        stream.write(input.as_bytes()).expect("failed to write");
    }
}
// TCPStream, TCPListenerしかない。TCPSoceketはない？

