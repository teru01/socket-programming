extern crate serde;
extern crate serde_json;

use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;
use std::io::prelude::*;

use serde_json::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Hoge {
    x: i32,
    s: String
}

fn main() {
    let remote = "127.0.0.1:33333".parse().unwrap();
    let mut stream = TcpStream::connect_timeout(&remote, Duration::from_secs(1)).expect("Could not connect.");
    stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    let hoge = Hoge {
        x: 10,
        s: "hoge".to_string()
    };

    loop {
        stream.write(serde_json::to_string(&hoge)).expect("failed to write");
    }
}
// TCPStream, TCPListenerしかない。TCPSoceketはない？

